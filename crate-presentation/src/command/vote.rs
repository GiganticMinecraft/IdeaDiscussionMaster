use crate_utils::{
    command::{
        builder::SlashCommandBuilder, force_boxed, ArgsMap, CommandInteraction, CommandResult,
        InteractionResponse,
    },
    SerenityContext,
};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("vote", "投票を行います。", Some(force_boxed(vote)))
}

async fn vote(
    _map: ArgsMap,
    _ctx: SerenityContext,
    _interaction: CommandInteraction,
) -> CommandResult {
    Ok(InteractionResponse::Message("vote".to_string()))
}
