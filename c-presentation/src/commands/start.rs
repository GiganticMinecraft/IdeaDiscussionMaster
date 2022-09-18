use crate::{
    commands::{CommandResult, Context},
    shared::{
        discord_embed,
        ext::{CreateEmbedExt, SortAgendasExt, UseFormattedId},
    },
};
use c_domain::id::{AgendaId, RecordId};

use anyhow::ensure;
use itertools::Itertools;
use log::{debug, info};
use poise::futures_util::future;

/// 会議を開始します
#[poise::command(slash_command)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "開始する会議議事録のチケット番号(省略した場合には最新のものが自動で使用されます)"]
    #[min = 1]
    record_id: Option<u16>,
) -> CommandResult {
    let data = ctx.data();
    ensure!(
        data.record_id.get().is_none(),
        "すでに会議が開始されているため、新しく会議を開始することはできません"
    );

    let vc_id = ctx
        .guild()
        .map(|g| g.voice_states)
        .and_then(|map| map.get(&ctx.author().id).cloned())
        .and_then(|state| state.channel_id)
        .ok_or_else(|| anyhow::anyhow!("会議を開始するにはVCに参加してください"))?;
    data.vc_id.save(vc_id);
    debug!("vc_id: {}", vc_id);

    let record_use_case = &data.use_cases.record;

    let record = match record_id.map(RecordId::new) {
        Some(id) => record_use_case.find_new(&id).await,
        None => record_use_case.find_latest_new().await,
    }?;
    let record_id = RecordId::new(record.id);
    data.record_id.save(record.id);
    info!("Discussion started: {}", record_id.formatted());

    let agendas = {
        let relations = record
            .relations
            .iter()
            .map(|id| AgendaId::new(id.to_owned()))
            .collect_vec();
        let agenda_use_case = &data.use_cases.agenda;

        let _ = future::join_all(
            relations
                .iter()
                .map(|id| async move { agenda_use_case.init(id).await }),
        )
        .await;

        agenda_use_case.list_new(&relations).await.sort_by_id()
    };

    info!(
        "Agendas({}): {:?}",
        agendas.len(),
        agendas
            .iter()
            .map(|agenda| AgendaId::new(agenda.id).formatted())
            .join(", ")
    );

    let next_agenda = agendas.first();
    if let Some(agenda) = next_agenda {
        info!("Next Agenda: {}", AgendaId::new(agenda.id).formatted());
        data.current_agenda_id.save(agenda.id);
    };

    let _ = ctx
        .send(|r| {
            r.embed(|e| {
                e.custom_default(&record_id)
                    .title("会議を開始しました")
                    .custom_field("議事録チケット", record.url(), false)
            })
            .embed(|e| match next_agenda {
                Some(agenda) => discord_embed::next_agenda_embed(e, &record_id, agenda),
                None => discord_embed::no_next_agenda(e, &record_id),
            })
        })
        .await;

    Ok(())
}
