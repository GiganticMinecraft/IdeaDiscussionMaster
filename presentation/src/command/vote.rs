use utils::command::{
    builder::SlashCommandBuilder, force_boxed, CommandArg, CommandResult, InteractionResponse,
};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("vote", "投票を行います。", Some(force_boxed(vote)))
}

async fn vote(_map: CommandArg) -> CommandResult {
    Ok(InteractionResponse::Message("vote".to_string()))
}
