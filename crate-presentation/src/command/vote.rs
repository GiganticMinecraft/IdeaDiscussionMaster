use crate_shared::command::{
    builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
    CommandResult, ExecutorArgs, InteractionResponse, SlashCommandChoice,
};

use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("vote", "投票を行います。")
        .add_option(SlashCommandOptionBuilder::new(
            "start",
            "投票を開始します。",
            ApplicationCommandOptionType::SubCommand,
        ))
        .add_option(
            SlashCommandOptionBuilder::new(
                "end",
                "指定したステータスで投票を終了します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "status",
                    "変更後のステータス",
                    ApplicationCommandOptionType::Integer,
                )
                .add_choice(("Approved", SlashCommandChoice::Int(1)))
                .add_choice(("Declined", SlashCommandChoice::Int(2)))
                .required(true),
            ),
        )
        .into()
}

pub async fn start((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("vote start".to_string()))
}

pub async fn end((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("vote end".to_string()))
}
