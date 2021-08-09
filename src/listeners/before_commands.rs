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
    let cached_record_id = {
        let cached_record_id = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<record_id::RecordId>()
                .expect("Expected RecordId in TypeMap.")
                .clone()
        };

        cached_record_id.load(Ordering::Relaxed)
    };
    if command_name == "start_discussion" && cached_record_id != 0 {
        message.reply(ctx, "会議はすでに始まっています。").await;

        return false;
    } else if command_name != "start_discussion" && cached_record_id == 0 {
        message.reply(ctx, "会議が始まっていません。").await;

        return false;
    }

    true
}
