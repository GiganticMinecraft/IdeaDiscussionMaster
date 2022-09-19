use crate::commands::{CommandResult, Context};

mod shared;

mod issue;
use issue::issue;

mod thread;
use thread::thread;

#[poise::command(slash_command, subcommands("issue", "thread"))]
pub async fn create(_: Context<'_>) -> CommandResult {
    Ok(())
}
