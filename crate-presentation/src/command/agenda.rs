use crate_shared::command::{
    builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
    CommandResult, ExecutorArgs, InteractionResponse, SlashCommandChoice,
};

use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("agenda", "議題の操作を行います。")
        .add_option(
            SlashCommandOptionBuilder::new(
                "add",
                "議題を追加します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "idea_issue_number",
                    "追加する議題のチケット番号",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(1)
                .required(true),
            ),
        )
        .add_option(SlashCommandOptionBuilder::new(
            "list",
            "議題の一覧を表示します。",
            ApplicationCommandOptionType::SubCommand,
        ))
        .add_option(
            SlashCommandOptionBuilder::new(
                "set",
                "議題のステータスを変更します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(SlashCommandOptionBuilder::new(
                "idea_issue_number",
                "ステータスを変更する議題のチケット番号",
                ApplicationCommandOptionType::Integer,
            ))
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

pub async fn add((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("add".to_string()))
}

pub async fn list((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("list".to_string()))
}

pub async fn set((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("set".to_string()))
}
