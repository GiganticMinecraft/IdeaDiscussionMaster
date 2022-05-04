use crate_shared::command::{
    builder::SlashCommandBuilder, CommandResult, ExecutorArgs, InteractionResponse,
};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("end", "アイデア会議を終了します。")
}

pub async fn executor((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message(
        "会議を終了しました。".to_string(),
    ))
}
