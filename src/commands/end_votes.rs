use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};
use std::str::FromStr;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use crate::domains::redmine_client::MockRedmineClient as RedmineClient;
    } else {
        pub use crate::domains::redmine_client::RedmineClient;
    }
}

use crate::{
    domains::{agenda_status, discord_embed, discussion, redmine_api},
    globals::{agendas, current_agenda_id, record_id, voted_message_id},
};

#[command]
#[aliases("evo","fvo")]
#[min_args(1)]
async fn end_votes(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
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

    //TODO: listener.rsとかぶっているのでまとめる

    voted_message_id::clear(&ctx).await;

    let record_id = record_id::read(&ctx).await;
    let current_agenda_id = current_agenda_id::read(&ctx).await;

    let _ = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                match status {
                    agenda_status::AgendaStatus::Approved => {
                        discord_embed::default_success_embed(embed, record_id)
                    }
                    agenda_status::AgendaStatus::Declined => {
                        discord_embed::default_failure_embed(embed, record_id)
                    }
                    _ => embed,
                }
                .title(format!(
                    "採決終了: #{}は{}されました",
                    current_agenda_id,
                    status.ja()
                ))
            })
        })
        .await;

    agendas::write(&ctx, current_agenda_id, status).await;
    current_agenda_id::clear(&ctx).await;

    let next_agenda_id = discussion::go_to_next_agenda(&ctx).await;
    let redmine_api = redmine_api::RedmineApi::new(RedmineClient::new());
    let next_redmine_issue = redmine_api
        .fetch_issue(&next_agenda_id.unwrap_or_default())
        .await
        .ok();

    let _ = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::next_agenda_embed(embed, record_id, next_redmine_issue)
            })
        })
        .await;

    Ok(())
}
