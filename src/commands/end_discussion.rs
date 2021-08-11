use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::Message, prelude::Context};

use crate::globals::{
    agendas, current_agenda_id, record_id, voice_chat_channel_id, voted_message_id,
};

// TODO: embed
// TODO: 結果をRedmineとDiscordに送信

#[command]
#[aliases("eid")]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    agendas::clear(ctx).await;
    current_agenda_id::clear(ctx).await;
    record_id::clear(ctx).await;
    voice_chat_channel_id::clear(ctx).await;
    voted_message_id::clear(ctx).await;

    message.reply(ctx, "会議を終了しました。").await?;

    Ok(())
}
