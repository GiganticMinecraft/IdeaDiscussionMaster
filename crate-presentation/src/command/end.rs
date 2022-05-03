use crate_utils::{
    command::{
        builder::SlashCommandBuilder, force_boxed, ArgsMap, CommandInteraction, CommandResult,
        InteractionResponse,
    },
    SerenityContext,
};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("end", "アイデア会議を終了します。", Some(force_boxed(end)))
}

async fn end(
    _map: ArgsMap,
    _ctx: SerenityContext,
    _interaction: CommandInteraction,
) -> CommandResult {
    Ok(InteractionResponse::Message("end".to_string()))
}
