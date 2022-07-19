use crate::{
    global,
    module::ModuleExt,
    shared::{
        command::{CommandResult, ExecutorArgs, InteractionResponse},
        discord_embeds,
        ext::{CommandExt, IdExt},
    },
};
use crate_domain::{error::MyError, id::IssueId, status::AgendaStatus};

use anyhow::{anyhow, ensure, Context};
use futures::stream::{self, StreamExt};
use itertools::Itertools;
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
    let ideas = ideas
        .split(' ')
        .filter_map(|str| str.parse::<u16>().ok())
        .map(IssueId::new)
        .filter(|id| record.relations.contains(id))
        .collect_vec();
    let ideas: Vec<_> = stream::iter(ideas)
        .then(|id| module.agenda_usecase().find(id))
        .collect()
        .await;
    let ideas = ideas
        .into_iter()
        .filter_map(|res| res.ok())
        .filter(|idea| idea.status == AgendaStatus::Approved)
        .collect_vec();

    debug!("ideas: {:?}", ideas);
    ensure!(
        !ideas.is_empty(),
        anyhow!("指定された議題は、いずれも存在しないか条件を満たしていません。")
    );

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
