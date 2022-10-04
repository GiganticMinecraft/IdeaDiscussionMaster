use super::shared::end_votes;
use crate::{
    commands::{CommandResult, Context},
    shared::{
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
    serenity_prelude::{CreateButton, InteractionResponseType, Message},
};
use std::{collections::HashMap, time::Duration};
use strum::IntoEnumIterator;

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
                        .title(format!("投票: {}", record_id.formatted()))
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

    let result_status = make_response_and_get_votes_result(&ctx, &vote_msg).await;
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
    ctx: &Context<'_>,
    msg: &Message,
) -> Option<AgendaStatus> {
    debug!("Start to response.");
    let mut res = None;
    let mut vote_map = HashMap::new();
    while let Some(interaction) = msg
        .await_component_interactions(&ctx.discord())
        .timeout(Duration::from_secs(VOTES_TIMEOUT_MINUTES * 60))
        .build()
        .next()
        .await
    {
        let _ = interaction.defer(&ctx.discord().http).await;

        let reacted_user = interaction
            .member
            .as_ref()
            .map(|member| member.user.to_owned());
        debug!(
            "Interaction is sent by {}",
            reacted_user.as_ref().unwrap().formatted_user_name()
        );
        let status = AgendaStatus::from_string(&interaction.data.custom_id).unwrap();
        debug!("Vote: {:?}", status);
        vote_map.insert(
            reacted_user
                .filter(|user| !user.bot)
                .map(|user| user.id)
                .unwrap(),
            status,
        );
        let _ = interaction
            .create_interaction_response(&ctx.discord(), |r| {
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

        let vc_members_count = {
            let vc_id = ctx.data().vc_id.get().unwrap();

            ctx.guild()
                .unwrap()
                .voice_states
                .iter()
                .filter(|(_, s)| s.channel_id.filter(|id| id == &vc_id).is_some())
                .count()
        };
        debug!("vc_members_count: {}", vc_members_count);

        let maybe_vote_result =
            total_votes(vote_map.clone().into_values().collect(), vc_members_count);
        debug!("maybe_vote_result: {:?}", maybe_vote_result);
        if maybe_vote_result.is_some() {
            res = maybe_vote_result;
            break;
        }
    }

    res
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
