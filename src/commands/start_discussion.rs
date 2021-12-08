use crate::{
    domains::{
        client::RedmineClient,
        custom_error::{DiscussionError, SpecifiedArgs},
        redmine,
    },
    globals::{
        agendas::{self, Agenda},
        record_id, voice_chat_channel_id,
    },
    utils::{discord_embed, discussion},
};
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[aliases("sid", "sdi")]
#[usage = "[議事録のチケット番号]"]
#[description = "会議を開始するコマンドです。\n議題の提示までを行います。"]
async fn start_discussion(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    // 引数に渡されたであろう番号の文字列をu16にparse。渡されていないかparseできなければ処理を中止。
    let record_id = match args.single::<u16>() {
        Ok(id) if id > 0 => id,
        _ => {
            return DiscussionError::ArgIsNotSpecified(SpecifiedArgs::TicketNumber).into();
        }
    };
    // 指定された番号の議事録チケットがあるかどうかRedmineのAPIを利用して確認。
    // Redmineと通信を行い、議事録チケットが存在したら、関連チケットのチケット番号をVecで返す。
    // Redmineとの通信でエラーが起きるor未実施の議事録チケットが存在しない場合は処理を中止。
    let redmine_client = RedmineClient::new();
    let record_relations = match redmine_client.fetch_issue_with_relations(record_id).await {
        Ok(issue) => {
            if issue.is_undone_idea_discussion_record() {
                issue.relations()
            } else {
                return DiscussionError::TicketIsNotFound.into();
            }
        }
        Err(err) => {
            return err.into();
        }
    };
    let record_relations = {
        let issues = stream::iter(record_relations)
            .then(|id| redmine_client.fetch_issue(id))
            .collect::<Vec<_>>()
            .await;
        issues
            .iter()
            .filter_map(|res| res.as_ref().ok())
            .filter(|issue| issue.is_undone_idea_ticket())
            .map(|issue| issue.id)
            .collect_vec()
    };

    if let Some(id) = discussion::fetch_voice_states(ctx, message.guild_id)
        .await
        .get(&message.author.id)
        .and_then(|state| state.channel_id)
    {
        voice_chat_channel_id::write(ctx, Some(id)).await;
    } else {
        return DiscussionError::VcIsNotJoined.into();
    }

    record_id::write(ctx, Some(record_id)).await;

    agendas::clear_all(ctx).await;
    for relation in record_relations.iter() {
        agendas::write(ctx, relation.to_owned(), Agenda::default()).await;
    }
    println!(
        "Agendas({}): {}",
        record_relations.len(),
        record_relations.iter().join(", ")
    );

    let _ = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::default_embed(embed, record_id)
                    .title("会議を開始しました")
                    .field(
                        "議事録チケット",
                        format!("{}/issues/{}", redmine::REDMINE_URL, record_id),
                        false,
                    )
            })
        })
        .await;

    println!("Discussion started: #{}", record_id);
    println!(
        "Agendas({}): {}",
        record_relations.len(),
        record_relations.iter().join(", ")
    );

    let next_agenda_id = discussion::go_to_next_agenda(ctx).await;
    let next_redmine_issue = redmine_client
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
    println!("Next agenda: #{}", next_agenda_id.unwrap_or_default());

    Ok(())
}
