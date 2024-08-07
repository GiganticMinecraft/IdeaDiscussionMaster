use super::shared::end_votes;
use crate::{
    commands::{CommandResult, Context},
    shared::{
        discord_embed,
        ext::{CreateEmbedExt, UseFormattedId, UseStatusEmoji},
        CommandError, VoteChoice, VoteChoiceWithId,
    },
};
use c_domain::redmine::model::{
    id::{AgendaId, RecordId},
    status::AgendaStatus,
};

use anyhow::{ensure, Context as _};
use itertools::Itertools;
use log::{debug, error, info};
use poise::{
    futures_util::StreamExt,
    serenity_prelude::{
        Attachment, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateMessage, EditMessage, Message, ReactionType,
    },
    CreateReply,
};
use std::{collections::HashMap, time::Duration};
use tokio::sync::{broadcast, mpsc};

/// 投票が無効になる制限時間
/// SlashCommandのdeferが15分なので、それよりも少し短い程度に
const VOTES_TIMEOUT_MINUTES: u64 = 13;

const VOTE_CHOICES_LIMIT: u8 = 5 * 5;

/// 採決を開始します
#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>, attachment: Option<Attachment>) -> CommandResult {
    let _ = ctx.defer().await;
    let votes = match attachment {
        Some(attachment) => {
            ensure!(
                attachment
                    .content_type
                    .as_ref()
                    .filter(|t| t.to_lowercase().contains("application/json"))
                    .is_some(),
                "The attachment must be json"
            );
            let attachment = attachment
                .download()
                .await
                .context("Failed to download the attachment")?;

            serde_json::from_slice::<Vec<VoteChoice>>(&attachment)
                .context("Failed to deserialize the attachment")?
                .into_iter()
                .unique()
                .enumerate()
                .collect_vec()
        }
        None => AgendaStatus::closed()
            .into_iter()
            .map(|s| VoteChoice::new(s, s.to_string()))
            .unique()
            .enumerate()
            .collect_vec(),
    }
    .into_iter()
    .map(|(id, choice)| (id + 1, choice))
    .collect_vec();
    ensure!(
        votes.len() <= VOTE_CHOICES_LIMIT.into(),
        "A vote can have up to {} choices",
        VOTE_CHOICES_LIMIT
    );

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

    let embed_description = ["提起されている議題についての投票を行います。",
        "以下のボタンで投票を行ってください。",
        "",
        "注意事項",
        "・過半数を超え次第、次の議題へと移ります。",
        "・複数回投票した場合は最後の投票が有効になります。",
        &format!(
            "・{}分以内に投票が終了しなければ、投票は無効となります。",
            VOTES_TIMEOUT_MINUTES
        ),
        "・「インタラクションに失敗した」というメッセージが表示されても、投票は正常に行われています。"]
    .join("\n");
    let vote_embed = CreateEmbed::new()
        .custom_default(&record_id)
        .title(format!("投票: {}", current_agenda_id.formatted()))
        .description(embed_description);

    let vote_msg = ctx
        .send(
            CreateReply::default()
                .embed(vote_embed)
                .embed(
                    CreateEmbed::new()
                        .custom_default(&record_id)
                        .title("投票選択肢")
                        .description(
                            votes
                                .iter()
                                .map(|(id, choice)| format!("選択肢{}: {}", id, choice))
                                .join("\n"),
                        ),
                )
                .components(
                    votes
                        .clone()
                        .into_iter()
                        .chunks(5)
                        .into_iter()
                        .map(|chunk| {
                            CreateActionRow::Buttons(
                                chunk
                                    .map(|(id, choice)| {
                                        CreateButton::new(id.to_string())
                                            .label(format!("選択肢{}", id))
                                            .emoji(ReactionType::from(choice.status.emoji()))
                                    })
                                    .collect_vec(),
                            )
                        })
                        .collect_vec(),
                ),
        )
        .await?
        .into_message()
        .await?;
    data.vote_message_id.save(vote_msg.id.get());
    debug!("vote_msg_id: {}", vote_msg.id);

    let votes_result = make_response_and_get_votes_result(ctx, vote_msg.clone(), votes).await;
    let _ = vote_msg.delete(&ctx.http()).await;
    match votes_result {
        Some(choice) => {
            end_votes(&ctx, choice).await?;
        }
        None => {
            error!("Interaction is timed out.");
            data.vote_message_id.clear();
            let _ = ctx
                .channel_id()
                .send_message(&ctx.http(), CreateMessage::new().content(format!("投票が{}分以内に終了しなかったため、投票は無効となりました。再度投票を行うには、`/vote start`コマンドを実行してください", VOTES_TIMEOUT_MINUTES)))
                // .send_message(&ctx.http(), |b| b.content(format!("投票が{}分以内に終了しなかったため、投票は無効となりました。再度投票を行うには、`/vote start`コマンドを実行してください", VOTES_TIMEOUT_MINUTES)))
                .await;
        }
    };

    Ok(())
}

/// 「投票メッセージへのインタラクションを受け取り、レスポンスをDiscordに送信した後、投票結果を計算する」を繰り返す
async fn make_response_and_get_votes_result(
    ctx: Context<'_>,
    msg: Message,
    vote_choices: Vec<VoteChoiceWithId>,
) -> Option<VoteChoice> {
    debug!("Start to response.");
    let (update_votes_snd, mut update_votes_recv) = broadcast::channel(20);
    let mut sub_update_votes_recv = update_votes_snd.subscribe();

    let serenity_ctx = ctx.serenity_context().clone();
    let wait_reactions = tokio::spawn(async move {
        debug!("Wait for reactions");
        let mut vote_map = HashMap::new();

        while let Some(interaction) = msg
            .await_component_interactions(&serenity_ctx)
            .timeout(Duration::from_secs(VOTES_TIMEOUT_MINUTES * 60))
            .stream()
            .next()
            .await
        {
            debug!("Receive interaction");
            let choice = vote_choices
                .iter()
                .find(|(id, _)| id == &interaction.data.custom_id.parse::<usize>().unwrap())
                .unwrap()
                .to_owned();
            debug!("Vote: {} {}", choice.0, choice.1);
            let _ = interaction
                .create_response(
                    &serenity_ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(format!(
                            "選択肢{}:「{}」に投票しました。2度目以降は最後の投票が有効になります",
                            choice.0, choice.1
                        ))
                            .ephemeral(true),
                    ),
                )
                .await;
            vote_map.insert(
                interaction
                    .member
                    .as_ref()
                    .filter(|member| !member.user.bot)
                    .map(|member| member.user.id)
                    .unwrap(),
                choice,
            );
            debug!("Send vote_map update");
            let _ = update_votes_snd.send(vote_map.clone().into_values().collect_vec());
        }

        debug!("Interaction is timeout");
    });

    let ch_id = ctx.channel_id();
    let serenity_ctx = ctx.serenity_context().clone();
    tokio::spawn(async move {
        let mut msg = ch_id
            .send_message(
                &serenity_ctx.http,
                CreateMessage::new()
                    .embed(discord_embed::vote_progress(CreateEmbed::new(), vec![])),
            )
            .await
            .unwrap();

        while let Ok(votes) = sub_update_votes_recv.recv().await {
            debug!("Update vote progress");
            let _ = msg
                .edit(
                    &serenity_ctx.http,
                    EditMessage::new()
                        .embed(discord_embed::vote_progress(CreateEmbed::new(), votes)),
                )
                .await;
        }
        let _ = msg.delete(&serenity_ctx.http).await;
    });

    let (votes_result_snd, mut votes_result_recv) = mpsc::channel(1);
    // TODO: use voice state
    let calculate_votes = tokio::spawn(async move {
        while let Ok(votes) = update_votes_recv.recv().await {
            debug!("Receive vote_map update");
            let vc_members_count = 0;
            debug!("vc_members_count: {}", vc_members_count);

            let maybe_vote_result = total_votes(votes, vc_members_count);
            debug!("maybe_vote_result: {:?}", maybe_vote_result);

            if let Some(res) = maybe_vote_result {
                debug!("Send found vote_result");
                let _ = votes_result_snd.send(res.1).await;
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

/// 票の配列とVCの参加人数を受け取り、人数の過半数を超える票が集まったものがないかを返す
fn total_votes(votes: Vec<VoteChoiceWithId>, vc_members_count: usize) -> Option<VoteChoiceWithId> {
    let counts = votes.into_iter().counts();
    debug!("votes_count_map: {:?}", counts);
    let half_of_total = vc_members_count / 2;
    debug!("half_of_total_members: {}", half_of_total);

    counts
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .filter(|(_, count)| count >= &half_of_total)
        .map(|(status, _)| status)
}
