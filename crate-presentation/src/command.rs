mod agenda;
mod create;
mod end;
mod start;
mod vote;

use crate_utils::command::{
    builder::{SlashCommandBuilder, SlashCommandBuilderExt},
    Executor,
};

use serenity::builder::CreateApplicationCommand;

fn all_builders() -> Vec<SlashCommandBuilder> {
    vec![
        agenda::builder(),
        create::builder(),
        end::builder(),
        start::builder(),
        vote::builder(),
    ]
}

pub fn executor<T: ToString>(command_name: T) -> Executor {
    let builder = all_builders();
    let builder = builder
        .into_iter()
        .find(|b| b.name == command_name.to_string())
        .unwrap();

    if builder.has_executor() {
        builder.executor.unwrap()
    } else {
        builder.sub_command().and_then(|b| b.executor).unwrap()
    }
}

pub fn all_commands() -> anyhow::Result<Vec<CreateApplicationCommand>> {
    all_builders().iter().map(|b| b.build()).collect()
}

pub fn all_command_names() -> Vec<String> {
    all_builders().iter().map(|b| b.name.to_owned()).collect()
}
