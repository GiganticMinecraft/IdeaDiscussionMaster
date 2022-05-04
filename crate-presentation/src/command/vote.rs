use crate_utils::command::{
    builder::SlashCommandBuilder, CommandResult, ExecutorArgs, InteractionResponse,
};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("vote", "投票を行います。")
}

pub async fn executor((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("vote".to_string()))
}
