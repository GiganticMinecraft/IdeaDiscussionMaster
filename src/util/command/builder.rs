use crate::command::Executor;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommandOption},
    model::interactions::application_command::ApplicationCommandOptionType,
};

#[derive(Clone, PartialEq)]
pub enum SlashCommandChoice {
    String(String),
    Int(i32),
    Number(f64),
}

type OptExecutor = Option<Executor>;

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

    pub fn add_option(&mut self, option: SlashCommandOptionBuilder) -> &mut Self {
        self.options.push(option);

        self
    }

    pub fn build(&self) -> CreateApplicationCommand {
        // 自分のOptionsにSubCommandを持たない限り、SubCommandはExecutorを持たなくてはいけない
        if self
            .options
            .iter()
            .all(|o| o.kind != ApplicationCommandOptionType::SubCommand)
        {
            assert!(self.has_executor());
        }

        let builder = &mut CreateApplicationCommand::default();
        builder.name(&self.name);
        builder.description(&self.description);

        self.options.iter().map(|o| o.build()).for_each(|o| {
            builder.add_option(o);
        });

        builder.to_owned()
    }
}

#[derive(Clone)]
pub struct SlashCommandOptionBuilder {
    builder: CreateApplicationCommandOption,
    pub name: String,
    pub description: String,
    pub kind: ApplicationCommandOptionType,
    pub choices: Vec<(String, SlashCommandChoice)>,
    pub options: Vec<Self>,
    pub executor: OptExecutor,
}

impl SlashCommandOptionBuilder {
    pub fn new<T: ToString>(
        name: T,
        description: T,
        kind: ApplicationCommandOptionType,
        executor: OptExecutor,
    ) -> Self {
        if kind == ApplicationCommandOptionType::SubCommand {
            assert!(executor.is_some())
        } else {
            assert!(executor.is_none())
        }

        Self {
            builder: CreateApplicationCommandOption::default(),
            name: name.to_string(),
            description: description.to_string(),
            kind,
            choices: vec![],
            options: vec![],
            executor,
        }
    }

    pub fn add_choice<T: ToString>(
        &mut self,
        (name, choice): (T, SlashCommandChoice),
    ) -> &mut Self {
        match choice {
            SlashCommandChoice::Int(_) => {
                assert_eq!(self.kind, ApplicationCommandOptionType::Integer)
            }
            SlashCommandChoice::Number(_) => {
                assert_eq!(self.kind, ApplicationCommandOptionType::Number)
            }
            SlashCommandChoice::String(_) => {
                assert_eq!(self.kind, ApplicationCommandOptionType::String)
            }
        }

        self.choices.push((name.to_string(), choice));

        self
    }

    pub fn required(&mut self, required: bool) -> &mut Self {
        self.builder = self.builder.required(required).to_owned();

        self
    }

    pub fn min_int(&mut self, value: i32) -> &mut Self {
        assert_eq!(self.kind, ApplicationCommandOptionType::Integer);

        self.builder = self.builder.min_int_value(value).to_owned();

        self
    }

    pub fn max_int(&mut self, value: i32) -> &mut Self {
        assert_eq!(self.kind, ApplicationCommandOptionType::Integer);

        self.builder = self.builder.max_int_value(value).to_owned();

        self
    }

    pub fn min_number(&mut self, value: f64) -> &mut Self {
        assert_eq!(self.kind, ApplicationCommandOptionType::Number);

        self.builder = self.builder.min_number_value(value).to_owned();

        self
    }

    pub fn max_number(&mut self, value: f64) -> &mut Self {
        assert_eq!(self.kind, ApplicationCommandOptionType::Number);

        self.builder = self.builder.max_number_value(value).to_owned();

        self
    }

    pub fn add_option(&mut self, value: &mut Self) -> &mut Self {
        // SubCommandはSubCommandを自身のOptionsに含められない
        if self.kind == ApplicationCommandOptionType::SubCommand {
            assert_ne!(value.kind, ApplicationCommandOptionType::SubCommand);
        }

        self.options.push(value.to_owned());

        self
    }

    pub fn build(&self) -> CreateApplicationCommandOption {
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

        self.options.iter().map(|o| o.build()).for_each(|o| {
            builder.add_sub_option(o);
        });

        builder.to_owned()
    }
}

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
