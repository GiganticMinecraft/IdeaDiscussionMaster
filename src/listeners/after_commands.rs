use serenity::{framework::standard::{CommandResult, macros::hook}, model::channel::Message, prelude::Context};

#[hook]
pub async fn after(ctx: &Context, message: &Message, command_name: &str, command_result: CommandResult) {

}