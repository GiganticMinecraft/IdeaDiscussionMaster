use crate::commands::{CommandResult, Context};

mod issue;
use issue::issue;

#[poise::command(slash_command, subcommands("issue"))]
pub async fn create(_: Context<'_>) -> CommandResult {
    Ok(())
}
