use serenity::{framework::standard::macros::hook, model::channel::Message, prelude::Context};
use std::sync::atomic::Ordering;

use crate::globals::record_id;

#[hook]
pub async fn before(ctx: &Context, message: &Message, command_name: &str) -> bool {
    println!(
        "Running command '{}' invoked by '{}'",
        command_name,
        message.author.tag()
    );

    if message.author.bot {
        return false;
    }

    let record_id_exists = record_id::read(ctx).await != 0;
    if command_name == "start_discussion" && record_id_exists {
        let _ = message.reply(ctx, "会議はすでに始まっています。").await;

        return false;
    } else if command_name != "start_discussion" && !record_id_exists {
        let _ = message.reply(ctx, "会議が始まっていません。").await;

        return false;
    }

    true
}
