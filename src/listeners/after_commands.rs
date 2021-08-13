use serenity::{
    framework::standard::{macros::hook, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[hook]
pub async fn after(
    ctx: &Context,
    message: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    if let Err(err) = command_result {
        let _ = message.reply(&ctx.http, &err).await;
        if format!("{}", err).contains("Fatal") {
            println!(
                "[{}] {}の処理中にエラーが発生しました。\nerror: {}\nmessage: {}\nauthor: {} (id: {})\nguild_id: {:?}",
                message.timestamp,
                command_name,
                err,
                message.content,
                message.author.name,
                message.author.id.as_u64(),
                message.guild_id
            );
        }
    }
}
