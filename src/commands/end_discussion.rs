use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::Message, prelude::Context};

use crate::globals::record_id;

// TODO: embed

#[command]
#[aliases("eid")]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    record_id::clear(ctx).await;
    message.reply(ctx, "会議を終了しました。").await?;

    Ok(())
}
