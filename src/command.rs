mod agenda;
mod create;
mod end;
mod start;
mod vote;

use crate::util::command::{
    application_interaction::ApplicationInteractions,
    builder::{SlashCommandBuilder, SlashCommandBuilderExt},
};
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use std::collections::HashMap;

fn all_builders() -> Vec<SlashCommandBuilder> {
    vec![
        agenda::builder(),
        create::builder(),
        end::builder(),
        start::builder(),
        vote::builder(),
    ]
}

pub type Executor =
    fn(HashMap<String, ApplicationInteractions>) -> anyhow::Result<InteractionResponse>;

pub enum InteractionResponse {
    Message(String),
    Embed(CreateEmbed),
}

pub fn executor<T: ToString>(command_name: T) -> Executor {
    let builder = all_builders();
    let builder = builder
        .iter()
        .find(|b| b.name == command_name.to_string())
        .unwrap();

    if builder.has_executor() {
        builder.executor.unwrap()
    } else {
        builder.sub_command().and_then(|b| b.executor).unwrap()
    }
}

pub fn all_commands() -> Vec<CreateApplicationCommand> {
    all_builders().iter().map(|b| b.build()).collect()
}

pub fn all_command_names() -> Vec<String> {
    all_builders().iter().map(|b| b.name.to_owned()).collect()
}
