use serenity::{framework::standard::macros::hook, model::channel::Message, prelude::Context};

#[hook]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    println!(
        "Running command '{}' invoked by '{}'",
        command_name,
        msg.author.tag()
    );

    true
}
