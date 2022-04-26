use super::{SlashCommandBuilder, SlashCommandOptionBuilder};
use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub trait SlashCommandBuilderExt {
    fn has_executor(&self) -> bool;
    fn sub_command(&self) -> Option<&SlashCommandOptionBuilder>;
}

impl SlashCommandBuilderExt for SlashCommandBuilder {
    fn has_executor(&self) -> bool {
        self.executor.is_some()
    }

    fn sub_command(&self) -> Option<&SlashCommandOptionBuilder> {
        self.options
            .iter()
            .find(|o| o.kind == ApplicationCommandOptionType::SubCommand)
    }
}

impl SlashCommandBuilderExt for SlashCommandOptionBuilder {
    fn has_executor(&self) -> bool {
        self.executor.is_some()
    }

    fn sub_command(&self) -> Option<&SlashCommandOptionBuilder> {
        self.options
            .iter()
            .find(|o| o.kind == ApplicationCommandOptionType::SubCommand)
    }
}
