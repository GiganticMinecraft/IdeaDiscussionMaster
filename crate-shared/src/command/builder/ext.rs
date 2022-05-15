use super::{SlashCommandBuilder, SlashCommandOptionBuilder};
use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub trait SlashCommandBuilderExt {
    fn sub_command(&self) -> Option<SlashCommandOptionBuilder>;
}

impl SlashCommandBuilderExt for SlashCommandBuilder {
    fn sub_command(&self) -> Option<SlashCommandOptionBuilder> {
        self.options
            .iter()
            .cloned()
            .find(|o| o.kind == ApplicationCommandOptionType::SubCommand)
    }
}

impl SlashCommandBuilderExt for SlashCommandOptionBuilder {
    fn sub_command(&self) -> Option<SlashCommandOptionBuilder> {
        self.options
            .iter()
            .cloned()
            .find(|o| o.kind == ApplicationCommandOptionType::SubCommand)
    }
}
