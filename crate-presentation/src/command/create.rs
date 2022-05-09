use super::super::{global, module::ModuleExt};
use crate_shared::{
    command::{
        builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
        CommandResult, ExecutorArgs, InteractionResponse,
    },
    ChronoExt, CreateEmbedExt,
};
use crate_usecase::model::RecordParam;

use anyhow::{anyhow, ensure, Context};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use itertools::Itertools;
use regex::Regex;
use serenity::{
    builder::CreateEmbed, model::interactions::application_command::ApplicationCommandOptionType,
};

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

pub async fn new_record((map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    let module = global::module::get();

    // 次回の会議の日付・時刻を取得
    let date = {
        let args: Vec<u16> = vec!["next_date_year", "next_date_month", "next_date_day"]
            .into_iter()
            .map(|str| map.get(str).cloned().unwrap().try_into().unwrap())
            .collect_vec();

        NaiveDate::from_ymd(args[0].into(), args[1].into(), args[2].into())
    };
    let start_time = {
        let args: Vec<u16> = vec![("next_time_hour", 21), ("next_time_minute", 0)]
            .into_iter()
            .map(|(str, default)| {
                map.get(str)
                    .cloned()
                    .map(|arg| arg.to_owned().try_into().unwrap())
                    .unwrap_or(default)
            })
            .collect_vec();

        NaiveTime::from_hms(args[0].into(), args[1].into(), 0)
    };
    let end_time = start_time + Duration::hours(2);

    // 次回の会議の日付・時刻を文字列にフォーマット
    let date_str = format!("{}({})", date.format("%Y/%m/%d"), date.weekday_ja());
    let time_formatter = "%H:%M";
    let start_time_str = start_time.format(time_formatter);
    let end_time_str = end_time.format(time_formatter);

    ensure!(
        Local::now().naive_local() <= NaiveDateTime::new(date, start_time),
        format!(
            "現在または現在より未来の日時を指定してください。: {} {}",
            date_str, start_time_str
        )
    );

    // 次回の会議の回数を取得
    let latest_closed_record_title = module.record_usecase().find_latest_closed().await?.title;
    let next_discussion_number = get_latest_record_number(latest_closed_record_title)
        .context("Error while getting latest record number")?
        + 1;

    // 議事録のタイトルと説明文を生成
    let record_title = format!("{}　第{}回アイデア会議", date_str, next_discussion_number);
    let record_description_date_time =
        format!("{}\n{}〜{}\n", date_str, start_time_str, end_time_str);
    let record_description = format!("{}{}", record_description_date_time, RECORD_DESCRIPTIONS);

    // 議事録をRedmine上に作成
    let new_record_param = RecordParam::new(record_title, record_description, None, Some(date));
    let new_record = module.record_usecase().add(new_record_param).await?;

    let embed = CreateEmbed::default()
        .title("議事録を新規作成しました")
        .field("タイトル", &new_record.title, false)
        .field("URL", new_record.url(), false)
        .current_timestamp()
        .success_color()
        .to_owned();

    Ok(InteractionResponse::Embed(embed))
}

fn get_latest_record_number(title: String) -> anyhow::Result<u16> {
    let regex = Regex::new(r"第([1-9][0-9]*)回").unwrap();
    let capture = regex
        .captures(&title)
        .ok_or_else(|| anyhow!("Failed to capture"))?;

    capture[1]
        .parse::<u16>()
        .map_err(|_| anyhow!("Failed to parse"))
}

pub async fn issue((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("issue".to_string()))
}

const RECORD_DESCRIPTIONS: &str = r"にアイデア会議を行います。

**開始前**
* 議題は予めこちらのチケットに注記を付加する形で提唱願います。
* ただし、運営チーム等が別途必要に応じて募集したアイデアに関しての議論は行いません。

**当日**
* 会議開始前に司会及び議事録作成係を決めて下さい。
* 議題順番は必要に応じて入れ替えてOKです。
* 議事録作成者は案件毎に「簡単な議論の流れ(後述する結論への補足として)」「結論(こういう方向で進みます、保留にします等)」「(結論でタスクが出来た場合)作業する担当者」「(ある場合)期日」などを議事録へ記載して下さい(内容は適宜調節してOKです。)
* 会議中は議事録作成者の負担を考慮して下さい。話を早く進めすぎると議事録への反映が漏れたり、異なる内容が記録される原因となります。必要に応じて議事録へ記載して欲しい内容を議事録作成者に伝えると良いと思います。

**終了後**
* 議事録作成係は作成した議事録を本チケットに注記を付加する形で投稿して下さい。
(会議に参加出来なかった方は後日議事録をご確認下さい)

**その他**
* 会議の進め方や心得が[[Wiki]]にまとまっていますのでご覧下さい。
* 議題として消化されたチケットは、運営チームが更新(クローズするなど)をしてください。";
