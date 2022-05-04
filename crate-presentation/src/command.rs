pub mod agenda;
pub mod create;
pub mod end;
pub mod start;
pub mod vote;

use crate_shared::command::builder::SlashCommandBuilder;

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

pub fn all_commands() -> anyhow::Result<Vec<CreateApplicationCommand>> {
    all_builders().iter().map(|b| b.build()).collect()
}

pub fn all_command_names() -> Vec<String> {
    all_builders().iter().map(|b| b.name.to_owned()).collect()
}
