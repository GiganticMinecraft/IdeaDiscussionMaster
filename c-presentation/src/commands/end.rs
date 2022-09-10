use crate::{
    commands::{CommandResult, Context},
    shared::ext::{SortAgendasExt, UseStatusJa},
};
use c_domain::id::{AgendaId, RecordId};
use c_usecase::model::AgendaDto;

use crate::shared::discord_embed;
use futures::future;
use itertools::Itertools;
use log::{debug, info};

/// 会議を終了します
#[poise::command(slash_command)]
pub async fn end(ctx: Context<'_>) -> CommandResult {
    let record_id = ctx
        .data()
        .record_id
        .get()
        .map(RecordId::new)
        .ok_or_else(|| anyhow::anyhow!("会議が開始されている必要があります"))?;
    let record = ctx.data().use_cases.record.find(&record_id).await?;
    let result = {
        let agenda_ids = record
            .relations
            .iter()
            .map(|id| AgendaId::new(id.to_owned()));

        future::join_all(
            agenda_ids.map(|id| async move { ctx.data().use_cases.agenda.find(&id).await }),
        )
        .await
        .into_iter()
        .filter_map(|agenda| agenda.ok())
        .collect_vec()
    }
    .sort_by_status();

    let result_strings = result
        .iter()
        .map(|(status, agendas)| {
            format!(
                "[{}]\n{}\n",
                status.ja(),
                agendas
                    .iter()
                    .map(|agenda| agenda.formatted_id())
                    .join(", ")
            )
        })
        .join("\n");
    // TODO: fix
    // ctx.data().use_cases.record
    //     .add_note(record_id, Note::from_string_content(result_strings.clone()))
    //     .await?;
    ctx.data().use_cases.record.close(&record_id).await?;

    info!("Discussion finished: {}", record.formatted_id());
    info!("Result:\n {}", result_strings);

    ctx.data().record_id.clear();
    ctx.data().vc_id.clear();

    let _ = ctx
        .send(|r| {
            r.embed(|e| {
                discord_embed::agendas_result(e, record, result).title("会議を終了しました")
            })
        })
        .await;

    Ok(())
}
