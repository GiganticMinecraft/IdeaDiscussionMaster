use crate::globals::records_id::RecordsId;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::*, prelude::*};
use std::sync::atomic::Ordering;

#[command]
#[aliases("sid")]
async fn start_discussion(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    let records_id = match args.single::<u16>() {
        Ok(id) if id > 0 => id,
        Ok(_) => {
            // TODO: 指定された議事録チケットがないことをRedmine経由で確認
            // そのとき、プロジェクトとトラッカーを必要条件とする
            message
                .reply(
                    ctx,
                    "指定された番号は議事録のチケット番号として適切ではありません。",
                )
                .await?;
            return Ok(());
        }
    let text_channel = match message.channel(&ctx.cache).await {
        Some(channel) => channel,
        None => {
            println!("テキストチャンネルを取得できませんでした。");
            message
                .reply(ctx, "内部エラーにより会議を開始できませんでした。")
                .await?;

            return Ok(());
        }
    };
    let vc_channel = match ctx.cache.guild_channel(vc_channel_id).await {
        Some(channel) => channel,
        None => {
            println!("VCチャンネルを取得できませんでした。");
            message
                .reply(ctx, "内部エラーにより会議を開始できませんでした。")
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
