use crate::shared::builder::{SlashCommandBuilder, SlashCommandOptionBuilder};

use serenity::model::interactions::application_command::ApplicationCommandOptionType;

mod issue;
pub use issue::issue;

mod thread;
pub use thread::thread;

mod new_record;
pub use new_record::new_record;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("create", "アイデア会議に関する様々なものを作成します。")
        .add_option(
            SlashCommandOptionBuilder::new(
                "issue",
                "承認された議題をGitHubのIssueとして作成します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "record_issue_number",
                    "Issueを作成する議題をもつ議事録のチケット番号",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(1)
                .required(true),
            )
            .add_option(SlashCommandOptionBuilder::new(
                "idea_issue_number_exceptions",
                "Issueを作成しない議題のチケット番号（半角スペース区切り）",
                ApplicationCommandOptionType::String,
            )),
        )
        .add_option(
            SlashCommandOptionBuilder::new(
                "thread",
                "承認された議題を議論するためのスレッドを作成します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "record_issue_number",
                    "スレッドを作成する議題をもつ議事録のチケット番号",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(1)
                .required(true),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "idea_issue_numbers",
                    "スレッドを作成する議題のチケット番号（半角スペース区切り）",
                    ApplicationCommandOptionType::String,
                )
                .required(true),
            ),
        )
        .add_option(
            SlashCommandOptionBuilder::new(
                "new_record",
                "議事録のチケットを新規作成します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "next_date_year",
                    "次回会議日付（年）",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(2000)
                .max_int(3000)
                .required(true),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "next_date_month",
                    "次回会議日付（月）",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(1)
                .max_int(12)
                .required(true),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "next_date_day",
                    "次回会議日付（日）",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(1)
                .max_int(31)
                .required(true),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "next_time_hour",
                    "次回会議開始時刻（時）, デフォルト: 21",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(0)
                .max_int(23),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "next_time_minute",
                    "次回会議開始時刻（分）, デフォルト: 0",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(0)
                .max_int(59),
            ),
        )
        .into()
}
