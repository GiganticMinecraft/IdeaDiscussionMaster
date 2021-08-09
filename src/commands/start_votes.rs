use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use std::sync::atomic::Ordering;

use crate::{
    domains::discord_embed,
    globals::{current_agenda_id, record_id},
};

#[command]
#[aliases("svo")]
pub async fn start_votes(ctx: &Context, message: &Message) -> CommandResult {
    let record_id = {
        let cached_record_id = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<record_id::RecordId>()
                .expect("Expected RecordId in TypeMap.")
                .clone()
        };
        cached_record_id.load(Ordering::Relaxed)
    };
    let current_agenda_id = {
        let cached_current_agenda_id = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<current_agenda_id::CurrentAgendaId>()
                .expect("Expected CurrentAgendaId in TypeMap.")
                .clone()
        };
        cached_current_agenda_id.load(Ordering::Relaxed)
    };
    let description = vec![
        "提議されている議題についての採決を行います。",
        "以下のリアクションで投票を行ってください。過半数を超え次第、次の議題へと移ります。",
        ":o:: 承認",
        ":x:: 却下",
    ]
    .join("\n");
    let current_agenda_exists = current_agenda_id != 0;

    let msg = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                if !current_agenda_exists {
                    discord_embed::default_failure_embed(embed, record_id)
                        .title("現在進行中の議題はありません")
                } else {
                    discord_embed::default_success_embed(embed, record_id)
                        .title(format!("採決: #{}", current_agenda_id))
                        .description(description)
                }
            })
        })
        .await?;

    if current_agenda_exists {
        msg.react(&ctx.http, '⭕').await?;
        msg.react(&ctx.http, '❌').await?;
    }

    Ok(())
}
