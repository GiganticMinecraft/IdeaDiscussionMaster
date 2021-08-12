use itertools::Itertools;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::Message, prelude::Context};

use crate::{
    domains::{discord_embed, redmine},
    globals::{agendas, current_agenda_id, record_id, voice_chat_channel_id, voted_message_id},
};

// TODO: 結果をRedmineに送信

#[command]
#[aliases("eid")]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    current_agenda_id::clear(ctx).await;
    voice_chat_channel_id::clear(ctx).await;
    voted_message_id::clear(ctx).await;

    let record_id = record_id::read(ctx).await;
    let age = agendas::read(ctx).await;
    let agendas = agendas::AgendaStatus::values()
        .into_iter()
        .map(|state| {
            let issue_ids = if let Some(array) = age
                .iter()
                .group_by(|(_, status)| **status == state)
                .into_iter()
                .map(|(boolean, group)| (boolean, group.collect_vec()))
                .find(|(boolean, _)| *boolean)
                .map(|(_, group)| group.into_iter().map(|(id, _)| id).collect_vec())
            {
                array.iter().map(|id| format!("#{}", id)).join(", ")
            } else {
                "-".to_string()
            };
            format!("[{}]\n{}", state.ja(), issue_ids)
        })
        .collect_vec()
        .join("\n");
    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::default_embed(embed, record_id)
                    .title("会議を終了しました")
                    .description(agendas)
                    .field(
                        "議事録チケット",
                        format!("{}{}", redmine::REDMINE_ISSUE_URL, record_id),
                        false,
                    )
            })
        })
        .await?;

    record_id::clear(ctx).await;
    agendas::clear(ctx).await;

    Ok(())
}
