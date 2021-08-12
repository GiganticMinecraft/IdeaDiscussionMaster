use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use crate::globals::{current_agenda_id, record_id, voted_message_id};

#[command]
#[aliases("evo")]
#[min_args(1)]
async fn end_votes(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    let status = if let Ok(str) = args.single::<String>() {
        str
    } else {
        return Err("ステータスが指定されていません。".into());
    };

    voted_message_id::clear(&ctx).await;

    let record_id = record_id::read(&ctx).await;
    let current_agenda_id = current_agenda_id::read(&ctx).await;

    // let _ = message
    //         .channel_id
    //         .send_message(&ctx.http, |msg| {
    //             msg.embed(|embed| {
    //                 match status_reaction {
    //                     AgendaStatus::Approved => {
    //                         discord_embed::default_success_embed(embed, record_id)
    //                     }
    //                     AgendaStatus::Declined => {
    //                         discord_embed::default_failure_embed(embed, record_id)
    //                     }
    //                     _ => embed,
    //                 }
    //                 .title(format!(
    //                     "採決終了: #{}は{}されました",
    //                     current_agenda_id,
    //                     status_reaction.ja()
    //                 ))
    //             })
    //         })
    //         .await;

    Ok(())
}
