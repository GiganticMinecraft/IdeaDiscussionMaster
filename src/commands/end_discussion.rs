use itertools::Itertools;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use strum::IntoEnumIterator;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use crate::domains::redmine_client::MockRedmineClient as RedmineClient;
    } else {
        pub use crate::domains::redmine_client::RedmineClient;
    }
}

use crate::{
    domains::{
        discord_embed, redmine_api,
        status::{agenda_status, record_status, trait_status::Status},
    },
    globals::{agendas, current_agenda_id, record_id, voice_chat_channel_id, voted_message_ids},
};

#[command]
#[aliases("eid", "edi")]
#[usage = "(引数なし)"]
#[description = "会議を終了するコマンドです。\n議事をまとめ、議事録を終了するまでを行います。"]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    current_agenda_id::clear(ctx).await;
    voice_chat_channel_id::clear(ctx).await;
    voted_message_ids::clear(ctx).await;

    let record_id = record_id::read(ctx).await;
    let cached_agendas = agendas::read(ctx).await;
    let agendas_result = agenda_status::AgendaStatus::iter()
        .map(|state| {
            let issue_ids = {
                let ids = cached_agendas
                    .iter()
                    .filter(|(_, agenda)| agenda.status == state)
                    .map(|(id, _)| id)
                    .collect_vec();
                if ids.is_empty() {
                    vec!["-".to_string()]
                } else {
                    ids.iter().map(|id| format!("#{}", id)).collect_vec()
                }
            };
            (state.ja(), issue_ids)
        })
        .collect_vec();
    let agendas_result_for_embed = agendas_result
        .iter()
        .map(|(state, array)| (state.to_owned(), array.join(", "), false))
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
                    .fields(agendas_result_for_embed)
            })
        })
        .await?;

    let agendas_result = agendas_result
        .iter()
        .map(|(status, issue_ids)| format!("[{}]\n{}\n", status, issue_ids.join(" ")))
        .collect_vec();
    let redmine_api = redmine_api::RedmineApi::new(RedmineClient::new());
    if let Err(err) = redmine_api.add_comments(record_id, agendas_result).await {
        return err.into();
    }
    if let Err(err) = redmine_api
        .update_issue_status(record_id, record_status::RecordStatus::Done.id())
        .await
    {
        return err.into();
    }

    record_id::clear(ctx).await;
    agendas::clear_all(ctx).await;

    Ok(())
}
