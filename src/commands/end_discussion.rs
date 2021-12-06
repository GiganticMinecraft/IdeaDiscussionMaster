use crate::{
    domains::{
        status::{AgendaStatus, RecordStatus},
        RedmineClient,
    },
    globals::{agendas, record_id, voice_chat_channel_id},
    utils::discord_embed,
};
use itertools::Itertools;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use strum::IntoEnumIterator;

#[command]
#[aliases("eid", "edi")]
#[usage = "(引数なし)"]
#[description = "会議を終了するコマンドです。\n議事をまとめ、議事録を終了するまでを行います。"]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    voice_chat_channel_id::clear(ctx).await;

    let record_id = record_id::read(ctx).await.unwrap();
    let cached_agendas = agendas::read(ctx).await;
    let agendas_result = AgendaStatus::iter()
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
                        format!("{}/issues/{}", redmine::REDMINE_URL, record_id),
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
    // let redmine_client = RedmineClient::new();
    // if let Err(err) = redmine_client.add_comments(record_id, agendas_result).await {
    //     return err.into();
    // }
    // if let Err(err) = redmine_client
    //     .update_issue_status(record_id, RecordStatus::Done.id())
    //     .await
    // {
    //     return err.into();
    // }

    println!("Discussion finished: #{}", record_id);
    println!("Result:\n {}", agendas_result.iter().join(", "));

    record_id::clear(ctx).await;
    agendas::clear_all(ctx).await;

    Ok(())
}
