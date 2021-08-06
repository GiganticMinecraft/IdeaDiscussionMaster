use crate::globals::MessageCount;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::sync::atomic::Ordering;

#[command]
async fn owo_count(ctx: &Context, msg: &Message) -> CommandResult {
    let raw_count = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageCount>()
            .expect("Expected MessageCount in TypeMap.")
            .clone()
    };

    let count = raw_count.load(Ordering::Relaxed);

    if count == 1 {
        msg.reply(
            ctx,
            "You are the first one to say owo this session! *because it's on the command name* :P",
        )
        .await?;
    } else {
        msg.reply(ctx, format!("OWO Has been said {} times!", count))
            .await?;
    }

    Ok(())
}
