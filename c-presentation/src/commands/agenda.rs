use crate::commands::{CommandResult, Context};

mod add;
use add::add;

#[poise::command(slash_command, subcommands("add"))]
pub async fn agenda(_: Context<'_>) -> CommandResult {
    Ok(())
}
