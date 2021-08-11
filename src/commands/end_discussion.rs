use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::Message, prelude::Context};

use crate::{
    domains::{discord_embed, redmine},
    globals::{agendas, current_agenda_id, record_id, voice_chat_channel_id, voted_message_id},
};

// TODO: 結果をRedmineとDiscordに送信

#[command]
#[aliases("eid")]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    agendas::clear(ctx).await;
    current_agenda_id::clear(ctx).await;
    voice_chat_channel_id::clear(ctx).await;
    voted_message_id::clear(ctx).await;

    let record_id = record_id::read(ctx).await;
    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::default_embed(embed, record_id)
                    .title("会議を終了しました")
                    .field(
                        "議事録チケット",
                        format!("{}{}", redmine::REDMINE_ISSUE_URL, record_id),
                        false,
                    )
            })
        })
        .await?;

    record_id::clear(ctx).await;

    Ok(())
}
