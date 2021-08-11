use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use std::sync::atomic::Ordering;

use crate::{
    domains::discord_embed,
    globals::{current_agenda_id, record_id, voted_message_id},
};

#[command]
#[aliases("svo")]
pub async fn start_votes(ctx: &Context, message: &Message) -> CommandResult {
    let record_id = record_id::read(ctx).await;
    let current_agenda_id = current_agenda_id::read(ctx).await;
    let current_agenda_exists = current_agenda_id != 0;
    let description = vec![
        "提議されている議題についての採決を行います。",
        "以下のリアクションで投票を行ってください。過半数を超え次第、次の議題へと移ります。",
        ":o:: 承認",
        ":x:: 却下",
    ]
    .join("\n");
    let voted_message = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                if !current_agenda_exists {
                    discord_embed::default_failure_embed(embed, record_id)
                        .title("現在進行中の議題はありません")
                } else {
                    discord_embed::default_embed(embed, record_id)
                        .title(format!("採決: #{}", current_agenda_id))
                        .description(description)
                }
            })
        })
        .await?;

    if current_agenda_exists {
        voted_message.react(&ctx.http, '⭕').await?;
        voted_message.react(&ctx.http, '❌').await?;

        let cached_voted_message_id = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<voted_message_id::VotedMessageId>()
                .expect("Expected VotedMessageId in TypeMap.")
                .clone()
        };
        cached_voted_message_id.store(voted_message.id.as_u64().to_owned(), Ordering::Relaxed);
    }

    Ok(())
}
