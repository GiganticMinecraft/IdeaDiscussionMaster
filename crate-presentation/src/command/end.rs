use crate_utils::command::{
    builder::SlashCommandBuilder, force_boxed, CommandArg, CommandResult, InteractionResponse,
};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("end", "アイデア会議を終了します。", Some(force_boxed(end)))
}

async fn end(_map: CommandArg) -> CommandResult {
    Ok(InteractionResponse::Message("end".to_string()))
}
