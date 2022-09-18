use crate::{
    commands::{CommandResult, Context},
    shared::{
        discord_embed,
        ext::{SortAgendasExt, UseFormattedId, UseStatusJa},
        CommandError,
    },
};
use c_domain::id::{AgendaId, RecordId};
use c_usecase::model::CreateNoteParam;

use itertools::Itertools;
use log::{debug, info};
use poise::futures_util::future;

/// 会議を終了します
#[poise::command(slash_command)]
pub async fn end(ctx: Context<'_>) -> CommandResult {
    let record_id = ctx
        .data()
        .record_id
        .get()
        .map(RecordId::new)
        .ok_or(CommandError::DiscussionHasBeenStarted)?;
    let record = ctx.data().use_cases.record.find(&record_id).await?;
    debug!("record_id: {}", record_id.formatted());
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
    .sort_and_grouping_by_status();

    let result_strings = result
        .iter()
        .map(|(status, agendas)| {
            format!(
                "[{}]\n{}\n",
                status.ja(),
                agendas
                    .iter()
                    .map(|agenda| AgendaId::new(agenda.id).formatted())
                    .join(", ")
            )
        })
        .collect_vec();

    info!("Discussion finished: {}", record_id.formatted());
    info!("Result:\n {}", result_strings.join("\n"));

    ctx.data()
        .use_cases
        .record
        .add_note(
            &record_id,
            CreateNoteParam::from_multi_line_string(result_strings),
        )
        .await?;
    ctx.data().use_cases.record.close(&record_id).await?;

    ctx.data().record_id.clear();
    ctx.data().vc_id.clear();
    ctx.data().current_agenda_id.clear();

    let _ = ctx
        .send(|r| {
            r.embed(|e| {
                discord_embed::agendas_result(e, record, result).title("会議を終了しました")
            })
        })
        .await;

    Ok(())
}
