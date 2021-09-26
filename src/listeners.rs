use serenity::{
    async_trait,
    framework::standard::{macros::hook, CommandResult},
    model::{channel::Message, id::RoleId, gateway::Ready},
    prelude::{Context, EventHandler},
};
use std::env;
use crate::globals::record_id;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[hook]
pub async fn before_commands(ctx: &Context, message: &Message, command_name: &str) -> bool {
    println!(
        "Running command '{}' invoked by '{}'",
        command_name,
        message.author.tag()
    );

    if message.author.bot {
        return false;
    }
    if message.guild_id.is_none() {
        return false;
    }
    if !message
        .author
        .has_role(
            &ctx.http,
            message.guild_id.unwrap(),
            RoleId::from(
                env::var("EXECUTABLE_ROLE_ID")
                    .ok()
                    .and_then(|str| str.parse::<u64>().ok())
                    .unwrap_or(0),
            ),
        )
        .await
        .ok()
        .unwrap_or(false)
    {
        let _ = message
            .reply(&ctx.http, "このコマンドを実行する権限がありません。")
            .await;

        return false;
    }

    if command_name == "help" {
        return true;
    }

    let record_id_exists = record_id::read(ctx).await.is_some();
    if command_name == "start_discussion" && record_id_exists {
        let _ = message.reply(ctx, "会議はすでに始まっています。").await;

        return false;
    } else if command_name != "start_discussion" && !record_id_exists {
        let _ = message.reply(ctx, "会議が始まっていません。").await;

        return false;
    }

    true
}

#[hook]
pub async fn after_commands(
    ctx: &Context,
    message: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    if let Err(err) = command_result {
        let _ = message.reply(&ctx.http, &err).await;
        if format!("{}", err).contains("不明なエラー") {
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
