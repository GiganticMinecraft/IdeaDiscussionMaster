use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
    utils::Colour,
};
use std::sync::atomic::Ordering;

use crate::{
    domains::redmine,
    globals::{
        agendas::{AgendaStatus, Agendas},
        record_id::RecordId,
    },
};

// TODO: エラーをまとめる
// TODO: 長くない？

#[command]
#[aliases("sid")]
async fn start_discussion(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    // 引数に渡されたであろう番号の文字列をu16にparse。渡されていないかparseできなければ処理を中止。
    let record_id = match args.single::<u16>() {
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
    let record = {
        match redmine::fetch_record_issue(record_id).await {
            Ok(issue) => {
                if record_id > 0
                    && issue.project.name == "アイデア会議議事録"
                    && issue.tracker.name == "アイデア会議"
                // && issue.status.name == "新規" FIXME: コメントアウト
                {
                    Some(issue)
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
    let record = match record {
        Some(issue) => issue,
        None => {
            message
                .reply(ctx, "指定された番号の議事録チケットが存在しません。")
                .await?;

            return Ok(());
        }
    };

    // FIXME: コメントアウト
    // let guild_id = match message.guild_id {
    //     Some(id) => id,
    //     None => {
    //         println!("会議を開始しようとしましたが、guild_idが見つかりませんでした。");
    //         message
    //             .reply(ctx, "内部エラーにより会議を開始できませんでした。")
    //             .await?;

    //         return Ok(());
    //     }
    // };

    // let guild = ctx.cache.guild(guild_id).await;
    // if guild.is_none() {
    //     println!(
    //         "会議を開始しようとしましたが、guildが見つかりませんでした。（guild_id: {}）",
    //         guild_id
    //     );
    //     message
    //         .reply(ctx, "内部エラーにより会議を開始できませんでした。")
    //         .await?;

    //     return Ok(());
    // }
    // match guild
    //     .unwrap()
    //     .voice_states
    //     .get(&message.author.id)
    //     .and_then(|state| state.channel_id)
    // {
    //     Some(id) => id,
    //     None => {
    //         message
    //             .reply(ctx, "会議を開始するにはVCに参加してください。")
    //             .await?;

    //         return Ok(());
    //     }
    // };

    {
        let cached_record_id = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<RecordId>()
                .expect("Expected RecordId in TypeMap.")
                .clone()
        };
        if cached_record_id.load(Ordering::Relaxed) != 0 {
            message.reply(ctx, "会議はすでに始まっています。").await?;

            return Ok(());
        }
        cached_record_id.store(record.id, Ordering::Relaxed);
    }

    {
        let cached_agendas = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<Agendas>()
                .expect("Expected Agendas in TypeMap.")
                .clone()
        };
        let mut agendas = cached_agendas.write().await;
        // TODO: 議題をフィルタしてsort
        record.relations.iter().for_each(|agenda| {
            agendas.insert(agenda.issue_id, AgendaStatus::New);
        });
        let cached_agendas_lock = cached_agendas.read().await;
        cached_agendas_lock.iter().for_each(|(id, status)| {
            println!("{} {:#?}", id, status);
        });
    }

    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                embed.title("会議を開始しました。");
                embed.field(
                    "議事録チケット",
                    format!("https://redmine.seichi.click/issues/{}", record.id),
                    false,
                );
                embed.colour(Colour::from_rgb(87, 199, 255));

                embed
            })
        })
        .await?;

    Ok(())
}