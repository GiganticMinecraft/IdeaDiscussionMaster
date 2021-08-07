use crate::globals::record_id::RecordId;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::*, prelude::*};
use std::sync::atomic::Ordering;

#[command]
#[aliases("eid")]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    let cached_record_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<RecordId>()
            .expect("Expected RecordId in TypeMap.")
            .clone()
    };

    if cached_record_id.load(Ordering::Relaxed) == 0 {
        message.reply(ctx, "会議が開始されていません。").await?;

        return Ok(());
    }

    cached_record_id.store(0, Ordering::Relaxed);
    message.reply(ctx, "会議を終了しました。").await?;

    Ok(())
}
