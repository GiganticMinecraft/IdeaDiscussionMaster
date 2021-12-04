use crate::{
    globals::{agendas, record_id},
    utils::discord_embed,
};
use itertools::Itertools;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[aliases("sha")]
#[description = "すべての議題の進行状況を表示します。"]
async fn show_agendas(ctx: &Context, message: &Message) -> CommandResult {
    let agendas = agendas::read(ctx).await;
    let agendas = agendas
        .iter()
        .map(|(id, agenda)| format!("#{}: {} {}", id, agenda.status.emoji(), agenda.status.ja()))
        .join("\n");
    let record_id = record_id::read(ctx).await.unwrap();
    let _ = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| discord_embed::default_embed(embed, record_id).description(agendas))
        })
        .await;

    Ok(())
}
