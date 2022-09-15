use crate::commands::{CommandResult, Context};

mod start;
use start::start;

mod end;
use end::end;

mod shared;

#[poise::command(slash_command, subcommands("start", "end"))]
pub async fn vote(_: Context<'_>) -> CommandResult {
    Ok(())
}
