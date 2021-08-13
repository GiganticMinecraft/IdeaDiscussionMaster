use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};
use std::str::FromStr;

use crate::{
    domains::agenda_status,
    globals::{current_agenda_id, record_id, voted_message_id},
};

#[command]
#[aliases("evo")]
#[min_args(1)]
async fn end_votes(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    // TODO: 任意の選択肢で手動投票を行った後、結果をBot側に入力するコマンド
    let status = if let Ok(str) = args.single::<String>() {
        if let Some(status) = agenda_status::AgendaStatus::from_str(&str)
            .ok()
            .or_else(|| agenda_status::AgendaStatus::from_ja(&str))
            .or_else(|| agenda_status::AgendaStatus::from_shorten(&str))
            .filter(|status| agenda_status::AgendaStatus::done_statuses().contains(status))
        {
            status
        } else {
            return Err("指定されたステータスは存在しないか、設定できません。".into());
        }
    } else {
        return Err("ステータスが指定されていません。".into());
    };

    voted_message_id::clear(&ctx).await;

    let record_id = record_id::read(&ctx).await;
    let current_agenda_id = current_agenda_id::read(&ctx).await;

    // let _ = message
    //     .channel_id
    //     .send_message(&ctx.http, |msg| {
    //         msg.embed(|embed| {
    //             match status_reaction {
    //                 AgendaStatus::Approved => {
    //                     discord_embed::default_success_embed(embed, record_id)
    //                 }
    //                 AgendaStatus::Declined => {
    //                     discord_embed::default_failure_embed(embed, record_id)
    //                 }
    //                 _ => embed,
    //             }
    //             .title(format!(
    //                 "採決終了: #{}は{}されました",
    //                 current_agenda_id,
    //                 status_reaction.ja()
    //             ))
    //         })
    //     })
    //     .await;

    Ok(())
}
