use super::{OptExecutor, SlashCommandBuilderExt, SlashCommandOptionBuilder};
use crate_domain::error::CommandBuilderError;

use anyhow::{bail, ensure};
use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::ApplicationCommandOptionType,
};

#[derive(Clone)]
pub struct SlashCommandBuilder {
    pub name: String,
    pub description: String,
    pub options: Vec<SlashCommandOptionBuilder>,
    pub executor: OptExecutor,
}

impl SlashCommandBuilder {
    pub fn new<T: ToString>(name: T, description: T, executor: OptExecutor) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            options: vec![],
            executor,
        }
    }

    pub fn add_option(&mut self, builder: impl Into<SlashCommandOptionBuilder>) -> &mut Self {
        self.options.push(builder.into());

        self
    }

    pub fn build(&self) -> anyhow::Result<CreateApplicationCommand> {
        // 自分のOptionsにSubCommandを持たない限り、SubCommandはExecutorを持たなくてはいけない
        if self
            .options
            .iter()
            .all(|o| o.kind != ApplicationCommandOptionType::SubCommand)
        {
            ensure!(
                self.has_executor(),
                CommandBuilderError::ExecutorIsNotDefined {
                    name: self.name.clone(),
                    description: self.description.clone()
                }
            );
        }

        let builder = &mut CreateApplicationCommand::default();
        builder.name(&self.name);
        builder.description(&self.description);

        for option in self.options.iter().map(|o| o.build()) {
            match option {
                Ok(o) => {
                    builder.add_option(o.to_owned());
                }
                Err(e) => {
                    bail!(e);
                }
            }
        }

        Ok(builder.to_owned())
    }
}

impl From<&mut SlashCommandBuilder> for SlashCommandBuilder {
    fn from(b: &mut Self) -> Self {
        b.to_owned()
    }
}
