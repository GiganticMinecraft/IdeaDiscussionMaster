use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};
use crate::{
    domains::{
        custom_error::{DiscussionError, SpecifiedArgs},
        status::AgendaStatus,
        RedmineClient
    },
    globals::{agendas, record_id},
    utils::{discord_embed, discussion},
};

#[command]
#[aliases("aag", "ada")]
#[usage = "[議題のチケット番号]"]
#[description = "議題を追加するコマンドです。\n議題の提示までを行います。"]
async fn add_agenda(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    // 引数に渡されたであろう番号の文字列をu16にparse。渡されていないかparseできなければ処理を中止。
    let issue_id = match args.single::<u16>() {
        Ok(id) if id > 0 => id,
        _ => {
            return DiscussionError::ArgIsNotSpecified(SpecifiedArgs::TicketNumber).into();
        }
    };
    let redmine_client = RedmineClient::new();
    let issue_id = match redmine_client.fetch_issue(issue_id).await {
        Ok(issue) => {
            if issue.is_undone_idea_ticket() {
                issue.id
            } else {
                return DiscussionError::ArgIsNotSpecified(SpecifiedArgs::TicketNumber).into();
            }
        }
        Err(err) => {
            return err.into();
        }
    };

    agendas::update_status(ctx, issue_id, AgendaStatus::New).await;

    let record_id = record_id::read(ctx).await.unwrap();
    if let Err(err) = redmine_client.add_relation(record_id, issue_id).await {
        return err.into();
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
    let next_redmine_issue = redmine_client
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
