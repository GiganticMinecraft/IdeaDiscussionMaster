use crate::{
    commands::{CommandResult, Context},
    shared::{
        discord_embed,
        ext::{SortAgendasExt, UseFormattedId, UseStatusJa},
        CommandError,
    },
};
use c_domain::redmine::model::id::{AgendaId, RecordId};
use c_usecase::redmine::model::CreateNoteParam;

use itertools::Itertools;
use log::{debug, info};
use poise::futures_util::future;

/// 会議を終了します
#[poise::command(slash_command)]
pub async fn end(ctx: Context<'_>) -> CommandResult {
    let _ = ctx.defer().await;
    let data = ctx.data();
    let record_use_case = &data.use_cases.record;
    let record_id = data
        .record_id
        .get()
        .map(RecordId::new)
        .ok_or(CommandError::DiscussionHasBeenStarted)?;
    let record = record_use_case.find(&record_id).await?;
    debug!("record_id: {}", record_id.formatted());
    let result = {
        let agenda_ids = record
            .relations
            .iter()
            .map(|id| AgendaId::new(id.to_owned()));

        future::join_all(agenda_ids.map(|id| async move { data.use_cases.agenda.find(&id).await }))
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

    record_use_case
        .add_note(
            &record_id,
            CreateNoteParam::from_multi_line_string(result_strings),
        )
        .await?;
    record_use_case.close(&record_id).await?;

    data.record_id.clear();
    data.vc_id.clear();
    data.current_agenda_id.clear();

    let _ = ctx
        .send(|r| {
            r.embed(|e| {
                discord_embed::agendas_result(e, record, result).title("会議を終了しました")
            })
        })
        .await;

    Ok(())
}
