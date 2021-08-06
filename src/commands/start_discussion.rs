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
        Err(_) => {
            message
                .reply(ctx, "議事録のチケット番号が指定されていません。")
                .await?;
            return Ok(());
        }
    };

    let is_author_joined_to_vc = ctx
        .cache
        .guild(message.guild_id.unwrap())
        .await
        .map(|guild| guild.voice_states.contains_key(&message.author.id))
        .unwrap_or(false);
    if !is_author_joined_to_vc {
        message
            .reply(ctx, "会議を開始するにはVCに参加してください。")
            .await?;

        return Ok(());
    }

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
    message.reply(ctx, "会議を開始しました。").await?;

    Ok(())
}
