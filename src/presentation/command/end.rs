use super::InteractionResponse;
use crate::util::command::builder::SlashCommandBuilder;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new(
        "end",
        "アイデア会議を終了します。",
        Some(|_map| Ok(InteractionResponse::Message("".to_string()))),
    )
}
