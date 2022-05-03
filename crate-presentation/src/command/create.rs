use crate_utils::{
    command::{
        builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
        force_boxed, ArgsMap, CommandInteraction, CommandResult, InteractionResponse,
    },
    SerenityContext,
};

use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new(
        "create",
        "アイデア会議に関する様々なものを作成します。",
        None,
    )
    .add_option(SlashCommandOptionBuilder::new(
        "new_record",
        "議事録のチケットを新規作成します。",
        ApplicationCommandOptionType::SubCommand,
        Some(force_boxed(new_record)),
    ))
    .add_option(
        SlashCommandOptionBuilder::new(
            "issue",
            "承認された議題をGitHubのIssueとして作成します。",
            ApplicationCommandOptionType::SubCommand,
            Some(force_boxed(issue)),
        )
        .add_option(
            SlashCommandOptionBuilder::new(
                "record_issue_number",
                "処理する議事録のチケット番号",
                ApplicationCommandOptionType::Integer,
                None,
            )
            .min_int(1)
            .required(true),
        )
        .add_option(
            SlashCommandOptionBuilder::new(
                "idea_issue_numbers",
                "Issueを作成する議題のチケット番号（コンマ区切り）",
                ApplicationCommandOptionType::String,
                None,
            )
            .required(true),
        ),
    )
    .into()
}

async fn new_record(
    _map: ArgsMap,
    _ctx: SerenityContext,
    _interaction: CommandInteraction,
) -> CommandResult {
    Ok(InteractionResponse::Message("new_record".to_string()))
}

async fn issue(
    _map: ArgsMap,
    _ctx: SerenityContext,
    _interaction: CommandInteraction,
) -> CommandResult {
    Ok(InteractionResponse::Message("issue".to_string()))
}
