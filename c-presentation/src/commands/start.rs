use crate::{
    commands::{CommandResult, Context},
    shared::{discord_embed, ext::CreateEmbedExt},
};
use c_domain::id::{AgendaId, RecordId};

use anyhow::ensure;
use itertools::Itertools;
use log::info;

/// 会議を開始します
#[poise::command(slash_command)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "開始する会議議事録のチケット番号(省略した場合には最新のものが自動で使用されます)"]
    #[min = 1]
    record_id: Option<u16>,
) -> CommandResult {
    ensure!(
        ctx.data().record_id.get().is_none(),
        "すでに会議は進行中です"
    );

    // TODO: remove comment out
    // let vc_id = ctx
    //     .guild()
    //     .map(|g| g.voice_states)
    //     .and_then(|map| map.get(&ctx.author().id).cloned())
    //     .and_then(|state| state.channel_id)
    //     .ok_or_else(|| anyhow::anyhow!("会議を開始するにはVCに参加してください"))?;
    // ctx.data().vc_id.save(ChannelId::new(vc_id.0));

    let record_use_case = &ctx.data().use_cases.record;

    let record = match record_id.map(RecordId::new) {
        Some(id) => record_use_case.find_new(id).await,
        None => record_use_case.find_latest_new().await,
    }?;
    let record_id = RecordId::new(record.id);
    ctx.data().record_id.save(record_id);

    let agendas = {
        let relations = record
            .relations
            .iter()
            .map(|id| AgendaId::new(id.to_owned()))
            .collect_vec();

        let mut result = Vec::new();
        for id in relations.iter() {
            let find_result = ctx.data().use_cases.agenda.find_new(id).await;
            if let Ok(dto) = find_result {
                result.push(dto)
            }
        }

        result
    };

    info!("Discussion started: {}", record.formatted_id());
    info!(
        "Agendas({}): {:?}",
        agendas.len(),
        agendas
            .iter()
            .map(|agenda| agenda.formatted_id())
            .join(", ")
    );

    let next_agenda = agendas.first();
    if let Some(agenda) = next_agenda {
        info!("Next Agenda: {}", agenda.formatted_id());
        ctx.data().current_agenda_id.save(AgendaId::new(agenda.id));
    };

    let _ = ctx
        .send(|r| {
            r.embed(|e| {
                e.custom_default(&record)
                    .title("会議を開始しました")
                    .custom_field("議事録チケット", record.url(), false)
            })
            .embed(|e| match next_agenda {
                Some(agenda) => discord_embed::next_agenda_embed(e, &record, agenda),
                None => discord_embed::no_next_agenda(e, &record),
            })
        })
        .await;

    Ok(())
}
