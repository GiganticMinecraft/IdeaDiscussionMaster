use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::*, id::ChannelId},
    prelude::*,
    utils::Colour,
};
use std::sync::atomic::Ordering;

use crate::{domains::redmine, globals::records_id::RecordsId};

// TODO: エラーをまとめる

#[command]
#[aliases("sid")]
async fn start_discussion(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    // 引数に渡されたであろう番号の文字列をu16にparse。渡されていないかparseできなければ処理を中止。
    let records_id = match args.single::<u16>() {
        Ok(id) => id,
        Err(_) => {
            message
                .reply(ctx, "議事録のチケット番号が指定されていません。")
                .await?;

            return Ok(());
        }
    };
    // 指定された番号の議事録チケットがあるかどうかRedmineのAPIを利用して確認。
    // Redmineとの通信でエラーが起きるor未実施の議事録チケットが存在しない場合はNone。
    let records_id = {
        match redmine::fetch_issue(records_id).await {
            Ok(issue) => {
                if records_id > 0
                    && issue.project.name == "アイデア会議議事録"
                    && issue.tracker.name == "アイデア会議"
                    && issue.status.name == "新規"
                {
                    Some(issue.id)
                } else {
                    None
                }
            }
            Err(err) => {
                println!("Redmineでのアクセス中にエラーが発生しました。: {}", err);

                None
            }
        }
    };
    // 番号が適切ではない場合のみ通知し、処理を中止。
    let records_id = match records_id {
        Some(id) => id,
        None => {
            message
                .reply(ctx, "指定された番号の議事録チケットが存在しません。")
                .await?;

            return Ok(());
        }
    };

    let cached_records_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<RecordsId>()
            .expect("Expected RecordsId in TypeMap.")
            .clone()
    };

    if cached_records_id.load(Ordering::Relaxed) != 0 {
        message.reply(ctx, "会議はすでに始まっています。").await?;

        return Ok(());
    }
    cached_records_id.store(records_id, Ordering::Relaxed);

    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                embed.title("会議を開始しました。");
                embed.field(
                    "議事録チケット",
                    format!("https://redmine.seichi.click/issues/{}", records_id),
                    false,
                );
                embed.colour(Colour::from_rgb(87, 199, 255));

                embed
            })
        })
        .await?;

    Ok(())
}
