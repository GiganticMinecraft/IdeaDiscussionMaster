use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use crate::domains::redmine_client::MockRedmineClient as RedmineClient;
    } else {
        pub use crate::domains::redmine_client::RedmineClient;
    }
}

use crate::{
    domains::{
        custom_error::DiscussionError, discord_embed, discussion, redmine_api,
        status::agenda_status,
    },
    globals::{agendas, record_id},
};

#[command]
#[aliases("aag", "ada")]
#[usage = "[議題のチケット番号]"]
#[min_args(1)]
#[description = "議題を追加するコマンドです。\n議題の提示までを行います。"]
async fn add_agenda(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    // 引数に渡されたであろう番号の文字列をu16にparse。渡されていないかparseできなければ処理を中止。
    let issue_id = match args.single::<u16>() {
        Ok(id) if id > 0 => id,
        _ => {
            return Err(DiscussionError::TicketNumberIsNotSpecified.to_string().into());
        }
    };
    let redmine_api = redmine_api::RedmineApi::new(RedmineClient::new());
    let issue_id = match redmine_api.fetch_issue(issue_id).await {
        Ok(issue) => {
            if issue.is_idea_ticket() {
                issue.id
            } else {
                return Err(DiscussionError::TickerIsNotFound.to_string().into());
            }
        }
        Err(err) => {
            return Err(err.to_string().into());
        }
    };

    agendas::write(ctx, issue_id, agenda_status::AgendaStatus::New).await;

    let record_id = record_id::read(ctx).await;
    if let Err(err) = redmine_api.add_relation(record_id, issue_id).await {
        return Err(err.to_string().into());
    };

    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::default_success_embed(embed, record_id)
                    .title("議題を追加しました")
                    .description("議題を再抽選し、表示し直します。")
            })
        })
        .await?;

    let next_agenda_id = discussion::go_to_next_agenda(ctx).await;
    let next_redmine_issue = redmine_api
        .fetch_issue(next_agenda_id.unwrap_or_default())
        .await
        .ok();
    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| discord_embed::next_agenda_embed(embed, issue_id, next_redmine_issue))
        })
        .await?;

    Ok(())
}
