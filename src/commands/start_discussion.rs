use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
    utils::Colour,
};
use std::sync::atomic::Ordering;

use crate::{
    domains::{discord_embed, discussion, redmine},
    globals::{agendas, record_id},
};

// TODO: エラーをまとめる
// TODO: 長くない？

#[command]
#[aliases("sid")]
async fn start_discussion(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    // 引数に渡されたであろう番号の文字列をu16にparse。渡されていないかparseできなければ処理を中止。
    let record_id = match args.single::<u16>() {
        Ok(id) if id > 0 => id,
        _ => {
            message
                .reply(ctx, "議事録のチケット番号が指定されていません。")
                .await?;

            return Ok(());
        }
    };
    // 指定された番号の議事録チケットがあるかどうかRedmineのAPIを利用して確認。
    // Redmineと通信を行い、議事録チケットが存在したら、関連チケットのチケット番号をSomeで包んでVecで返す。
    // Redmineとの通信でエラーが起きるor未実施の議事録チケットが存在しない場合はNone。
    let record_relations = match redmine::fetch_record_issue(record_id).await {
        Ok(issue) => {
            if issue.project.name == "アイデア会議議事録" && issue.tracker.name == "アイデア会議"
            // && issue.status.name == "新規" // FIXME: コメントアウト
            {
                let relations = issue
                    .relations
                    .iter()
                    .filter(|rel| rel.relation_type == "relates")
                    .flat_map(|rel| [rel.issue_id, rel.issue_to_id])
                    .filter(|num| num != &issue.id)
                    .collect::<Vec<_>>();

                Some(relations)
            } else {
                None
            }
        }
        Err(err) => {
            println!("Redmineでのアクセス中にエラーが発生しました。: {}", err);

            None
        }
    };
    // 番号が適切ではない場合のみ通知し、処理を中止。
    let record_relations = if let Some(relations) = record_relations {
        relations
    } else {
        message
            .reply(ctx, "指定された番号の議事録チケットが存在しません。")
            .await?;

        return Ok(());
    };

    // FIXME: コメントアウト
    // let guild_id = match message.guild_id {
    //     Some(id) => id,
    //     None => {
    //         println!("会議を開始しようとしましたが、guild_idが見つかりませんでした。");
    //         message
    //             .reply(ctx, "内部エラーにより会議を開始できませんでした。")
    //             .await?;

    //         return Ok(());
    //     }
    // };

    // let guild = ctx.cache.guild(guild_id).await;
    // if guild.is_none() {
    //     println!(
    //         "会議を開始しようとしましたが、guildが見つかりませんでした。（guild_id: {}）",
    //         guild_id
    //     );
    //     message
    //         .reply(ctx, "内部エラーにより会議を開始できませんでした。")
    //         .await?;

    //     return Ok(());
    // }
    // match guild
    //     .unwrap()
    //     .voice_states
    //     .get(&message.author.id)
    //     .and_then(|state| state.channel_id)
    // {
    //     Some(id) => id,
    //     None => {
    //         message
    //             .reply(ctx, "会議を開始するにはVCに参加してください。")
    //             .await?;

    //         return Ok(());
    //     }
    // };

    {
        let cached_record_id = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<record_id::RecordId>()
                .expect("Expected RecordId in TypeMap.")
                .clone()
        };
        cached_record_id.store(record_id, Ordering::Relaxed);
    }

    {
        let cached_agendas = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<agendas::Agendas>()
                .expect("Expected Agendas in TypeMap.")
                .clone()
        };
        let mut agendas = cached_agendas.write().await;

        // TODO: 議題などのクリアは会議終了時にもされるべき
        agendas.clear();
        record_relations.iter().for_each(|agenda_id| {
            agendas.insert(agenda_id.to_owned(), agendas::AgendaStatus::New);
        });
    }

    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::default_success_embed(embed, record_id)
                    .title("会議を開始しました")
                    .field(
                        "議事録チケット",
                        format!("{}{}", redmine::REDMINE_ISSUE_URL, record_id),
                        false,
                    )
            })
        })
        .await?;

    let next_agenda_id = discussion::go_to_next_agenda(ctx).await;
    message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| match next_agenda_id {
                Some(id) => discord_embed::default_success_embed(embed, record_id)
                    .title(format!("次の議題は#{}です", id))
                    .field(
                        "議題チケット",
                        format!("{}{}", redmine::REDMINE_ISSUE_URL, id),
                        false,
                    )
                    .colour(Colour::from_rgb(87, 199, 255)),
                None => discord_embed::default_failure_embed(embed, record_id)
                    .title("次の議題はありません")
                    .description("Redmine上で提起されていた議題は全て処理されました。"),
            })
        })
        .await?;

    Ok(())
}
