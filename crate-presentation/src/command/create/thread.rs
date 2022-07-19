use crate::{
    global,
    module::ModuleExt,
    shared::{
        command::{CommandResult, ExecutorArgs, InteractionResponse},
        discord_embeds,
        ext::{CommandExt, IdExt},
        issue_id_array_parser::refine_all_approved_agendas,
    },
};
use crate_domain::{error::MyError, id::IssueId};

use anyhow::Context;
use log::{debug, info};

pub async fn thread((map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let module = global::module::get();

    info!("Create threads");

    // 議事録のIDを取得
    let record_id: u16 = map
        .get("record_issue_number")
        .ok_or_else(|| MyError::ArgIsNotFound("record_issue_number".to_string()))?
        .to_owned()
        .try_into()?;
    let record = module
        .record_usecase()
        .find(IssueId::new(record_id))
        .await
        .with_context(|| format!("議事録の取得中にエラーが発生しました: #{:?}", record_id))?;

    // スレッドを作成するアイデアを取得
    // ただし、以下をすべて満たす必要がある
    // * u16にパースできる
    // * 議事録に関連付けられている
    // * ステータスが承認である
    let ideas: String = map
        .get("idea_issue_numbers")
        .ok_or_else(|| MyError::ArgIsNotFound("idea_issue_numbers".to_string()))?
        .to_owned()
        .try_into()?;
    let ideas = refine_all_approved_agendas(ideas, &record.relations, &module).await?;

    let base_msg = interaction
        .send(
            &ctx.http,
            InteractionResponse::Message("スレッドを作成しました。".to_string()),
        )
        .await?;

    info!("Start to create threads per each idea");
    for idea in ideas.iter() {
        debug!("Create thread of {}", idea.id.formatted());

        if let Ok(th) = interaction
            .channel_id
            .create_public_thread(&ctx.http, base_msg.id, |b| {
                // Threads will be archived in 24 hours automatically
                b.name(format!(
                    "{}: {}",
                    record.discussion_title(),
                    idea.id.formatted()
                ))
                .auto_archive_duration(60 * 24)
            })
            .await
        {
            let _ = th
                .send_message(&ctx.http, |b|
                    b.content(format!(
                        "このスレッドは、{}にて承認されたアイデアについて個別に議論を行うためのものです。",
                         record.discussion_title()
                    )).embed(|e|
                        discord_embeds::next_agenda_embed(e, &record.id, idea)
                            .title(format!("このスレッドで議論を行う議題は{}です", idea.id.formatted()))
                    )
                )
                .await;
        }
    }

    Ok(())
}
