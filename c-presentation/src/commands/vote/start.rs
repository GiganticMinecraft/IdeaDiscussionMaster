use super::shared::end_votes;
use crate::{
    commands::{CommandResult, Context},
    shared::{
        discord_embed,
        ext::{CreateEmbedExt, UseFormattedId, UseFormattedUserName, UseStatusEmoji, UseStatusJa},
        CommandError,
    },
};
use c_domain::redmine::model::{
    id::{AgendaId, RecordId},
    status::AgendaStatus,
};

use anyhow::ensure;
use itertools::Itertools;
use log::{debug, error, info};
use poise::{
    futures_util::StreamExt,
    serenity_prelude::{CreateButton, InteractionResponseType, Message, VoiceState},
};
use std::{collections::HashMap, time::Duration};
use strum::IntoEnumIterator;
use tokio::sync::{broadcast, mpsc};

/// 投票が無効になる制限時間
/// SlashCommandのdeferが15分なので、それよりも少し短い程度に
const VOTES_TIMEOUT_MINUTES: u64 = 13;

/// 採決を開始します
#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>) -> CommandResult {
    let data = ctx.data();
    let record_id = data
        .record_id
        .get()
        .map(RecordId::new)
        .ok_or(CommandError::DiscussionHasBeenStarted)?;
    let current_agenda_id = data
        .current_agenda_id
        .get()
        .map(AgendaId::new)
        .ok_or(CommandError::AgendaIsNotFound)?;
    ensure!(
        data.vote_message_id.get().is_none(),
        "すでに投票を開始しています。ログを確認してください"
    );

    info!("Vote started: {}", current_agenda_id.formatted());

    let embed_description = vec![
        "提起されている議題についての投票を行います。",
        "以下のボタンで投票を行ってください。",
        "",
        "注意事項",
        "・過半数を超え次第、次の議題へと移ります。",
        "・複数回投票した場合は最後の投票が有効になります。",
        &format!(
            "・{}分以内に投票が終了しなければ、投票は無効となります。",
            VOTES_TIMEOUT_MINUTES
        ),
        "・「インタラクションに失敗した」というメッセージが表示されても、投票は正常に行われています。"
    ]
    .join("\n");

    let vote_msg = ctx
        .send(|reply| {
            reply
                .embed(|embed| {
                    embed
                        .custom_default(&record_id)
                        .title(format!("投票: {}", current_agenda_id.formatted()))
                        .description(embed_description)
                })
                .components(|c| {
                    c.create_action_row(|row| {
                        AgendaStatus::iter()
                            .filter(|status| status.is_closed())
                            .map(|status| {
                                CreateButton::default()
                                    .custom_id(status.to_string())
                                    .label(format!("{}: {}", status.emoji(), status.ja()))
                                    .to_owned()
                            })
                            .for_each(|button| {
                                row.add_button(button);
                            });

                        row
                    })
                })
        })
        .await?
        .into_message()
        .await?;
    data.vote_message_id.save(vote_msg.id.0);
    debug!("vote_msg_id: {}", vote_msg.id);

    let result_status = make_response_and_get_votes_result(ctx, vote_msg.clone()).await;
    let _ = vote_msg.delete(&ctx.discord().http).await;
    match result_status {
        Some(status) => {
            end_votes(&ctx, status).await?;
        }
        None => {
            error!("Interaction is timed out.");
            data.vote_message_id.clear();
            let _ = ctx
                .channel_id()
                .send_message(&ctx.discord().http, |b| b.content(format!("投票が{}分以内に終了しなかったため、投票は無効となりました。再度投票を行うには、`/vote start`コマンドを実行してください", VOTES_TIMEOUT_MINUTES)))
                .await;
        }
    };

    Ok(())
}

/// 「投票メッセージへのインタラクションを受け取り、レスポンスをDiscordに送信した後、投票結果を計算する」を繰り返す
async fn make_response_and_get_votes_result(
    ctx: Context<'_>,
    msg: Message,
) -> Option<AgendaStatus> {
    debug!("Start to response.");
    let (update_votes_snd, mut update_votes_recv) = broadcast::channel(20);
    let mut sub_update_votes_recv = update_votes_snd.subscribe();

    let serenity_ctx = ctx.discord().clone();
    let wait_reactions = tokio::spawn(async move {
        debug!("Wait for reactions");
        let mut vote_map = HashMap::new();

        while let Some(interaction) = msg
            .await_component_interactions(&serenity_ctx)
            .timeout(Duration::from_secs(VOTES_TIMEOUT_MINUTES * 60))
            .build()
            .next()
            .await
        {
            let _ = interaction.defer(&serenity_ctx.http).await;

            debug!("Receive interaction");
            let status = AgendaStatus::from_string(&interaction.data.custom_id).unwrap();
            debug!("Vote: {:?}", status);
            let _ = interaction
                .create_interaction_response(&serenity_ctx.http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {
                            d.content(format!(
                                "「{}」に投票しました。2度目以降は最後の投票が有効になります",
                                status.ja()
                            ))
                            .ephemeral(true)
                        })
                })
                .await;
            vote_map.insert(
                interaction
                    .member
                    .as_ref()
                    .filter(|member| !member.user.bot)
                    .map(|member| member.user.id)
                    .unwrap(),
                status,
            );
            debug!("Send vote_map update");
            let _ = update_votes_snd.send(vote_map.clone().into_values().collect_vec());
        }

        debug!("Interaction is timeout");
    });

    let ch_id = ctx.channel_id();
    let serenity_ctx = ctx.discord().clone();
    tokio::spawn(async move {
        let mut msg = ch_id
            .send_message(&serenity_ctx.http, |c| {
                c.embed(|e| discord_embed::vote_progress(e, vec![]))
            })
            .await
            .unwrap();

        while let Ok(votes) = sub_update_votes_recv.recv().await {
            debug!("Update vote progress");
            let _ = msg
                .edit(&serenity_ctx.http, |c| {
                    c.embed(|e| discord_embed::vote_progress(e, votes))
                })
                .await;
        }
        let _ = msg.delete(&serenity_ctx.http).await;
    });

    let (votes_result_snd, mut votes_result_recv) = mpsc::channel(1);
    let voice_states = ctx.guild().unwrap().voice_states;
    let vc_id = ctx.data().vc_id.get().unwrap();
    let calculate_votes = tokio::spawn(async move {
        while let Ok(votes) = update_votes_recv.recv().await {
            debug!("Receive vote_map update");
            let vc_members_count = voice_states
                .iter()
                .filter(|(_, s)| s.channel_id.filter(|id| id == &vc_id).is_some())
                .count();
            debug!("vc_members_count: {}", vc_members_count);

            let maybe_vote_result = total_votes(votes, vc_members_count);
            debug!("maybe_vote_result: {:?}", maybe_vote_result);

            if let Some(res) = maybe_vote_result {
                debug!("Send found vote_result");
                let _ = votes_result_snd.send(res).await;
            }
        }
    });

    while !wait_reactions.is_finished() && !calculate_votes.is_finished() {
        debug!("Start to receive votes_result");
        if let Some(result) = votes_result_recv.recv().await {
            calculate_votes.abort();
            wait_reactions.abort();

            return Some(result);
        }
    }

    None
}

/// AgendaStatusの配列とVCの参加人数を受け取り、人数の過半数を超える票が集まったものがないかを返す
fn total_votes(vec: Vec<AgendaStatus>, vc_members_count: usize) -> Option<AgendaStatus> {
    let counts = vec.into_iter().counts();
    debug!("votes_count_map: {:?}", counts);
    let half_of_total = vc_members_count / 2;
    debug!("half_of_total_members: {}", half_of_total);

    counts
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .filter(|(_, count)| count >= &half_of_total)
        .map(|(status, _)| status)
}
