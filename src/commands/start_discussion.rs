use itertools::Itertools;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
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
    domains::{agenda_status, discord_embed, discussion, redmine_api},
    globals::{agendas, record_id, voice_chat_channel_id},
};

#[command]
#[aliases("sid", "bid")]
#[min_args(1)]
async fn start_discussion(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    // 引数に渡されたであろう番号の文字列をu16にparse。渡されていないかparseできなければ処理を中止。
    let record_id = match args.single::<u16>() {
        Ok(id) if id > 0 => id,
        _ => {
            return Err("議事録のチケット番号が指定されていません。".into());
        }
    };
    // 指定された番号の議事録チケットがあるかどうかRedmineのAPIを利用して確認。
    // Redmineと通信を行い、議事録チケットが存在したら、関連チケットのチケット番号をVecで返す。
    // Redmineとの通信でエラーが起きるor未実施の議事録チケットが存在しない場合は処理を中止。
    let redmine_api = redmine_api::RedmineApi::new(RedmineClient::new());
    let record_relations = match redmine_api.fetch_issue_with_relations(&record_id).await {
        Ok(issue) => {
            if issue.project.name == "アイデア会議議事録" && issue.tracker.name == "アイデア会議"
            // && issue.status.name == "新規" // FIXME: コメントアウト
            {
                issue
                    .relations
                    .iter()
                    .filter(|rel| rel.relation_type == "relates")
                    .flat_map(|rel| [rel.issue_id, rel.issue_to_id])
                    .filter(|num| num != &issue.id)
                    .collect_vec()
            } else {
                return Err("指定された番号の議事録チケットが存在しません。".into());
            }
        }
        Err(err) => {
            return Err(format!("Redmineへのアクセス中にエラーが発生しました。管理者に連絡してください。\nFatalError: {:?}", err).into());
        }
    };

    let vc_id = ChannelId(872720546742296667);
    voice_chat_channel_id::write(ctx, vc_id.as_u64().to_owned()).await;
    // FIXME: コメントアウト
    // if let Some(id) = discussion::fetch_voice_states(ctx, message.guild_id)
    //     .await
    //     .get(&message.author.id)
    //     .and_then(|state| state.channel_id)
    // {
    //     voice_chat_channel_id::write(ctx, id.as_u64().to_owned()).await;
    // } else {
    //     message
    //         .reply(ctx, "会議を開始するにはVCに参加してください。")
    //         .await?;

    //     return Ok(());
    // }

    record_id::write(ctx, record_id).await;

    agendas::clear(&ctx).await;
    for relation in record_relations.iter() {
        agendas::write(&ctx, relation.to_owned(), agenda_status::AgendaStatus::New).await;
    }

    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::default_embed(embed, record_id)
                    .title("会議を開始しました")
                    .field(
                        "議事録チケット",
                        format!("{}/issues/{}", redmine_api::REDMINE_URL, record_id),
                        false,
                    )
            })
        })
        .await?;

    let next_agenda_id = discussion::go_to_next_agenda(ctx).await;
    let next_redmine_issue = redmine_api
        .fetch_issue(&next_agenda_id.unwrap_or_default())
        .await
        .ok();
    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::next_agenda_embed(embed, record_id, next_redmine_issue)
            })
        })
        .await?;

    Ok(())
}
