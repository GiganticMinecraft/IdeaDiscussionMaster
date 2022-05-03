use super::SlashCommandOptionBuilder;

use anyhow::bail;
use serenity::builder::CreateApplicationCommand;

#[derive(Clone)]
pub struct SlashCommandBuilder {
    pub name: String,
    pub description: String,
    pub options: Vec<SlashCommandOptionBuilder>,
}

impl SlashCommandBuilder {
    pub fn new<T: ToString>(name: T, description: T) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            options: vec![],
        }
    }

    pub fn add_option(&mut self, builder: impl Into<SlashCommandOptionBuilder>) -> &mut Self {
        self.options.push(builder.into());

        self
    }

    pub fn build(&self) -> anyhow::Result<CreateApplicationCommand> {
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
