pub mod application_interaction;
pub mod builder;

mod interaction_response;
pub use interaction_response::InteractionResponse;

mod command_ext;
pub use command_ext::CommandExt;

mod parser;
pub use parser::Parser;

mod slash_command_choice;
pub use slash_command_choice::SlashCommandChoice;

// https://stackoverflow.com/questions/66769143/rust-passing-async-function-pointers

use application_interaction::ApplicationInteractions;
use std::collections::HashMap;

pub type ArgsMap = HashMap<String, ApplicationInteractions>;
pub type CommandInteraction =
    serenity::model::interactions::application_command::ApplicationCommandInteraction;

pub type ExecutorArgs = (ArgsMap, super::SerenityContext, CommandInteraction);
pub type CommandResult = anyhow::Result<InteractionResponse>;
