use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::*, prelude::*};
use std::sync::atomic::Ordering;
use crate::globals::records_id::RecordsId;

#[command]
#[aliases("eid")]
async fn end_discussion(ctx: &Context, message: &Message) -> CommandResult {
    let cached_records_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<RecordsId>()
            .expect("Expected RecordsId in TypeMap.")
            .clone()
    };

    if cached_records_id.load(Ordering::Relaxed) == 0 {
        message.reply(ctx, "会議が開始されていません。").await?;

        return Ok(());
    }

    cached_records_id.store(0, Ordering::Relaxed);
    message.reply(ctx, "会議を終了しました。").await?;

    Ok(())
}
