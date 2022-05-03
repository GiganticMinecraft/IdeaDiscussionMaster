use crate_utils::{
    command::{
        builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
        force_boxed, ArgsMap, CommandInteraction, CommandResult, InteractionResponse,
        SlashCommandChoice,
    },
    SerenityContext,
};

use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("agenda", "議題の操作を行います。", None)
        .add_option(
            SlashCommandOptionBuilder::new(
                "add",
                "議題を追加します。",
                ApplicationCommandOptionType::SubCommand,
                Some(force_boxed(add)),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "idea_issue_number",
                    "追加する議題のチケット番号",
                    ApplicationCommandOptionType::Integer,
                    None,
                )
                .min_int(1)
                .required(true),
            ),
        )
        .add_option(SlashCommandOptionBuilder::new(
            "list",
            "議題の一覧を表示します。",
            ApplicationCommandOptionType::SubCommand,
            Some(force_boxed(list)),
        ))
        .add_option(
            SlashCommandOptionBuilder::new(
                "set",
                "議題のステータスを変更します。",
                ApplicationCommandOptionType::SubCommand,
                Some(force_boxed(set)),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "status",
                    "変更後のステータス",
                    ApplicationCommandOptionType::Integer,
                    None,
                )
                .add_choice(("Approved", SlashCommandChoice::Int(1)))
                .add_choice(("Declined", SlashCommandChoice::Int(2)))
                .required(true),
            ),
        )
        .into()
}

async fn add(
    _map: ArgsMap,
    _ctx: SerenityContext,
    _interaction: CommandInteraction,
) -> CommandResult {
    Ok(InteractionResponse::Message("add".to_string()))
}

async fn list(
    _map: ArgsMap,
    _ctx: SerenityContext,
    _interaction: CommandInteraction,
) -> CommandResult {
    Ok(InteractionResponse::Message("list".to_string()))
}

async fn set(
    _map: ArgsMap,
    _ctx: SerenityContext,
    _interaction: CommandInteraction,
) -> CommandResult {
    Ok(InteractionResponse::Message("set".to_string()))
}
