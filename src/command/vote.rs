use crate::{command::InteractionResponse, util::command::builder::SlashCommandBuilder};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new(
        "vote",
        "投票を行います。",
        Some(|_map| Ok(InteractionResponse::Message("".to_string()))),
    )
    .to_owned()
}
