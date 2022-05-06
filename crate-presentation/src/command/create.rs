use crate_shared::command::{
    builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
    CommandResult, ExecutorArgs, InteractionResponse,
};

use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("create", "アイデア会議に関する様々なものを作成します。")
        .add_option(
            SlashCommandOptionBuilder::new(
                "new_record",
                "議事録のチケットを新規作成します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "next_date",
                    "次回の会議の日付",
                    ApplicationCommandOptionType::String,
                )
                .required(true),
            )
            .add_option(SlashCommandOptionBuilder::new(
                "next_start_time",
                "次回の会議の開始時刻",
                ApplicationCommandOptionType::String,
            )),
        )
        .add_option(
            SlashCommandOptionBuilder::new(
                "issue",
                "承認された議題をGitHubのIssueとして作成します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "record_issue_number",
                    "処理する議事録のチケット番号",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(1)
                .required(true),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "idea_issue_numbers",
                    "Issueを作成する議題のチケット番号（コンマ区切り）",
                    ApplicationCommandOptionType::String,
                )
                .required(true),
            ),
        )
        .into()
}

pub async fn new_record((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("new_record".to_string()))
}

pub async fn issue((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("issue".to_string()))
}
