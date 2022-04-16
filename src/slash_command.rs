use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommandOption},
    model::interactions::application_command::ApplicationCommandOptionType,
};

#[derive(Clone)]
pub enum SlashCommandChoice {
    String(String),
    Int(i32),
    Number(f64),
}

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

    pub fn add_option(&mut self, option: SlashCommandOptionBuilder) -> &mut Self {
        self.options.push(option);

        self
    }

    pub fn build(&self) -> CreateApplicationCommand {
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
}

impl SlashCommandOptionBuilder {
    pub fn new<T: ToString>(name: T, description: T, kind: ApplicationCommandOptionType) -> Self {
        Self {
            builder: CreateApplicationCommandOption::default(),
            name: name.to_string(),
            description: description.to_string(),
            kind,
            choices: vec![],
            options: vec![],
        }
    }

    pub fn add_choice(&mut self, choice: (String, SlashCommandChoice)) -> &mut Self {
        self.choices.push(choice);

        self
    }

    pub fn required(&mut self, required: bool) -> &mut Self {
        self.builder = self.builder.required(required).to_owned();

        self
    }

    pub fn min_int(&mut self, value: i32) -> &mut Self {
        self.builder = self.builder.min_int_value(value).to_owned();

        self
    }

    pub fn max_int(&mut self, value: i32) -> &mut Self {
        self.builder = self.builder.max_int_value(value).to_owned();

        self
    }

    pub fn min_number(&mut self, value: f64) -> &mut Self {
        self.builder = self.builder.min_number_value(value).to_owned();

        self
    }

    pub fn max_number(&mut self, value: f64) -> &mut Self {
        self.builder = self.builder.max_number_value(value).to_owned();

        self
    }

    pub fn add_option(&mut self, value: &mut Self) -> &mut Self {
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
