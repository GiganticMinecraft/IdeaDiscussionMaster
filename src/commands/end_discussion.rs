use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::Message, prelude::Context};
use std::sync::atomic::Ordering;

use crate::globals::record_id;

// TODO: embed

#[command]
#[aliases("eid")]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    let cached_record_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<record_id::RecordId>()
            .expect("Expected RecordId in TypeMap.")
            .clone()
    };
    cached_record_id.store(0, Ordering::Relaxed);
    message.reply(ctx, "会議を終了しました。").await?;

    Ok(())
}
