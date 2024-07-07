use poise::CreateReply;

use crate::commands::{CommandResult, Context};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> CommandResult {
    let _ = ctx.send(CreateReply::default().content("Pong!")).await?;

    Ok(())
}
