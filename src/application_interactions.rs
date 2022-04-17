use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOptionValue;

#[derive(Debug, Clone)]
pub enum SlashCommand {
    Command(String),
    SubCommand(String),
    Option(Box<ApplicationCommandInteractionDataOptionValue>),
}

#[derive(Debug, Clone)]
pub enum ApplicationInteractions {
    SlashCommand(SlashCommand),
}
