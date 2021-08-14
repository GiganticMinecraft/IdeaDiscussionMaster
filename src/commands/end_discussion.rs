use itertools::Itertools;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use strum::IntoEnumIterator;

use crate::{
    domains::{agenda_status, discord_embed, redmine_api},
    globals::{agendas, current_agenda_id, record_id, voice_chat_channel_id, voted_message_id},
};

// TODO: 結果をRedmineに送信

#[command]
#[aliases("eid")]
#[usage = "(引数なし)"]
#[description = "会議を終了するコマンドです。\n議事をまとめ、議事録を終了するまでを行います。"]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    current_agenda_id::clear(ctx).await;
    voice_chat_channel_id::clear(ctx).await;
    voted_message_id::clear(ctx).await;

    let record_id = record_id::read(ctx).await;
    let cached_agendas = agendas::read(ctx).await;
    let agendas_result = agenda_status::AgendaStatus::iter()
        .map(|state| {
            let issue_ids = if let Some(array) = cached_agendas
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
            (state.ja(), issue_ids, false)
        })
        .collect_vec();
    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::default_embed(embed, record_id)
                    .title("会議を終了しました")
                    .field(
                        "議事録チケット",
                        format!("{}/issues/{}", redmine_api::REDMINE_URL, record_id),
                        false,
                    )
                    .fields(agendas_result)
            })
        })
        .await?;

    record_id::clear(ctx).await;
    agendas::clear(ctx).await;

    Ok(())
}
