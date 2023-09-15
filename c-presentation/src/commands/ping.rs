use crate::commands::{CommandResult, Context};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> CommandResult {
    let _ = ctx.send(|b| b.content("pong!")).await;

    Ok(())
}
