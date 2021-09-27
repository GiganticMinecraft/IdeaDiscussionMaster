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
    domains::{
        custom_error::{DiscussionError, SpecifiedArgs},
        discord_embed, discussion, redmine_api,
        status::{agenda_status::AgendaStatus, trait_status::Status},
    },
    globals::{agendas, record_id},
};

#[command]
#[aliases("evo")]
#[usage = "[議題ステータス(app, dec)]"]
#[min_args(1)]
#[description = "投票を終了するコマンドです。\n選択肢が所定のもの以外の場合は、このコマンドを使用して議論結果を入力してください。"]
pub async fn end_votes(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    let status = if let Ok(str) = args.single::<String>() {
        if let Some(status) = AgendaStatus::from_str(&str)
            .ok()
            .or_else(|| AgendaStatus::from_ja(&str))
            .or_else(|| AgendaStatus::from_alias(&str))
            .filter(|status| status.is_done())
        {
            status
        } else {
            return DiscussionError::ArgIsNotSpecified(SpecifiedArgs::TicketStatus).into();
        }
    } else {
        return DiscussionError::ArgIsNotSpecified(SpecifiedArgs::TicketStatus).into();
    };

    let record_id = record_id::read(ctx).await.unwrap();
    let current_agenda_id = if let Some(id) = agendas::find_current_agenda_id(ctx).await {
        id
    } else {
        let _ = message
            .channel_id
            .send_message(&ctx.http, |msg| {
                msg.embed(|embed| discord_embed::no_current_agenda_embed(embed, record_id))
            })
            .await?;
        return Ok(());
    };

    if let Some(id) = agendas::find_votes_message_id(ctx, current_agenda_id).await {
        let _ = message.channel_id.delete_message(&ctx.http, id).await;
        agendas::update_votes_message_id(ctx, current_agenda_id, None).await;
    }

    let _ = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::votes_result_embed(embed, record_id, current_agenda_id, status)
            })
        })
        .await;

    let redmine_api = redmine_api::RedmineApi::new(RedmineClient::new());
    if let Err(err) = redmine_api
        .update_issue_status(current_agenda_id, status.id())
        .await
    {
        return err.into();
    }

    agendas::update_status(ctx, current_agenda_id, status).await;

    let next_agenda_id = discussion::go_to_next_agenda(ctx).await;
    let next_redmine_issue = redmine_api
        .fetch_issue(next_agenda_id.unwrap_or_default())
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
