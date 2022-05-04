use super::super::SlashCommandChoice;
use crate_domain::error::{CommandBuilderError, CommandInfo};

use anyhow::{bail, ensure};
use serenity::{
    builder::CreateApplicationCommandOption,
    model::interactions::application_command::ApplicationCommandOptionType,
};

type OptionType = ApplicationCommandOptionType;

#[derive(Clone)]
pub struct SlashCommandOptionBuilder {
    builder: CreateApplicationCommandOption,
    pub name: String,
    pub description: String,
    pub kind: OptionType,
    pub choices: Vec<(String, SlashCommandChoice)>,
    pub options: Vec<Self>,
}

// assertがいくつかあるが、build時には確認できないものばかりなのでそのまま
impl SlashCommandOptionBuilder {
    pub fn new<T: ToString>(name: T, description: T, kind: OptionType) -> Self {
        Self {
            builder: CreateApplicationCommandOption::default(),
            name: name.to_string(),
            description: description.to_string(),
            kind,
            choices: vec![],
            options: vec![],
        }
    }

    pub fn add_choice<T: ToString>(
        &mut self,
        (name, choice): (T, SlashCommandChoice),
    ) -> &mut Self {
        self.choices.push((name.to_string(), choice));

        self
    }

    pub fn required(&mut self, required: bool) -> &mut Self {
        self.builder = self.builder.required(required).to_owned();

        self
    }

    pub fn min_int(&mut self, value: i32) -> &mut Self {
        assert_eq!(self.kind, OptionType::Integer);

        self.builder = self.builder.min_int_value(value).to_owned();

        self
    }

    pub fn max_int(&mut self, value: i32) -> &mut Self {
        assert_eq!(self.kind, OptionType::Integer);

        self.builder = self.builder.max_int_value(value).to_owned();

        self
    }

    pub fn min_number(&mut self, value: f64) -> &mut Self {
        assert_eq!(self.kind, OptionType::Number);

        self.builder = self.builder.min_number_value(value).to_owned();

        self
    }

    pub fn max_number(&mut self, value: f64) -> &mut Self {
        assert_eq!(self.kind, OptionType::Number);

        self.builder = self.builder.max_number_value(value).to_owned();

        self
    }

    pub fn add_option(&mut self, builder: impl Into<Self>) -> &mut Self {
        let builder = builder.into();
        // SubCommandはSubCommandを自身のOptionsに含められない
        if self.kind == ApplicationCommandOptionType::SubCommand {
            assert_ne!(builder.kind, ApplicationCommandOptionType::SubCommand);
        }
        self.options.push(builder);

        self
    }

    pub fn build(&self) -> anyhow::Result<CreateApplicationCommandOption> {
        self.assert()?;

        let builder = &mut self.builder.to_owned();
        builder.name(&self.name);
        builder.description(&self.description);
        builder.kind(self.kind);

        self.choices.iter().for_each(|(name, choice)| {
            match choice {
                SlashCommandChoice::String(value) => builder.add_string_choice(name, value),
                SlashCommandChoice::Int(value) => builder.add_int_choice(name, *value),
                SlashCommandChoice::Number(value) => builder.add_number_choice(name, *value),
            };
        });

        for option in self.options.iter().map(|o| o.build()) {
            match option {
                Ok(o) => {
                    builder.add_sub_option(o.to_owned());
                }
                Err(e) => {
                    bail!(e);
                }
            }
        }

        Ok(builder.to_owned())
    }

    fn assert(&self) -> anyhow::Result<()> {
        // OptionとChoiceの型チェック
        for (choice_name, choice) in self.choices.iter() {
            match choice {
                SlashCommandChoice::Int(_) => {
                    ensure!(
                        self.kind == OptionType::Integer,
                        CommandBuilderError::ChoiceAndOptionTypeMisMatch {
                            command: CommandInfo::new(self.name.clone(), self.description.clone()),
                            choice_name: choice_name.to_owned(),
                            choice: OptionType::Integer
                        }
                    )
                }
                SlashCommandChoice::Number(_) => {
                    ensure!(
                        self.kind == OptionType::Number,
                        CommandBuilderError::ChoiceAndOptionTypeMisMatch {
                            command: CommandInfo::new(self.name.clone(), self.description.clone()),
                            choice_name: choice_name.to_owned(),
                            choice: OptionType::Number
                        }
                    )
                }
                SlashCommandChoice::String(_) => {
                    ensure!(
                        self.kind == OptionType::String,
                        CommandBuilderError::ChoiceAndOptionTypeMisMatch {
                            command: CommandInfo::new(self.name.clone(), self.description.clone()),
                            choice_name: choice_name.to_owned(),
                            choice: OptionType::String
                        }
                    )
                }
            }
        }

        Ok(())
    }
}

impl From<&mut SlashCommandOptionBuilder> for SlashCommandOptionBuilder {
    fn from(b: &mut Self) -> Self {
        b.to_owned()
    }
}
