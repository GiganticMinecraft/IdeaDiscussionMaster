use crate::{
    global,
    module::ModuleExt,
    shared::{
        command::{CommandResult, ExecutorArgs, InteractionResponse},
        ext::{ChronoExt, CommandExt, CreateEmbedExt},
    },
};
use crate_domain::error::MyError;
use crate_usecase::model::{DtoExt, RecordParam};

use anyhow::{ensure, Context};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use itertools::Itertools;
use log::{debug, info};
use serenity::builder::CreateEmbed;

pub async fn new_record((map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let module = global::module::get();

    info!("Create new record");

    // 次回の会議の日付・時刻を取得
    let date = {
        let arg_strs = vec!["next_date_year", "next_date_month", "next_date_day"];
        let mut args = Vec::new();
        for str in arg_strs.into_iter() {
            let arg: u16 = map
                .get(str)
                .cloned()
                .ok_or_else(|| MyError::ArgIsNotFound(str.to_string()))?
                .try_into()
                .unwrap();
            args.push(arg);
        }

        NaiveDate::from_ymd(args[0].into(), args[1].into(), args[2].into())
    };
    let start_time = {
        let args: Vec<u16> = vec![("next_time_hour", 21), ("next_time_minute", 0)]
            .into_iter()
            .map(|(str, default)| {
                map.get(str)
                    .cloned()
                    .map(|arg| arg.try_into().unwrap())
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

    info!("{} {} - {}", date_str, start_time_str, end_time_str);
    ensure!(
        Local::now().naive_local() <= NaiveDateTime::new(date, start_time),
        format!(
            "現在または現在より未来の日時を指定してください。: {} {}",
            date_str, start_time_str
        )
    );

    // 次回の会議の回数を取得
    let latest_closed_discussion_number = module
        .record_usecase()
        .find_latest_closed()
        .await
        .context("No closed record")?
        .discussion_number()
        .context("Error while getting latest record number")?;
    let next_discussion_number = latest_closed_discussion_number + 1;

    debug!(
        "latest {} -> next {}",
        latest_closed_discussion_number, next_discussion_number
    );

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
        .custom_field("タイトル", &new_record.title, false)
        .custom_field("URL", new_record.url(), false)
        .current_timestamp()
        .success_color()
        .to_owned();

    interaction
        .send(&ctx.http, InteractionResponse::Embed(embed))
        .await
        .map(|_| ())
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
