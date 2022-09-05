use crate::commands::{CommandResult, Context};

/// Shows buttons to add or remove slash commands. (prefix command)
#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> CommandResult {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}