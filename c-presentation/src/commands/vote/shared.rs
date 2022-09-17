use crate::{
    commands::Context,
    shared::{
        discord_embed,
        ext::{SortAgendasExt, UseFormattedId},
        CommandError,
    },
};
use c_domain::{
    id::{AgendaId, RecordId},
    status::AgendaStatus,
};

use itertools::Itertools;
use log::info;

pub async fn end_votes(ctx: &Context<'_>, status: AgendaStatus) -> anyhow::Result<()> {
    info!("Vote finished: {:?}", status);
    let record_id = ctx
        .data()
        .record_id
        .get()
        .map(RecordId::new)
        .ok_or(CommandError::DiscussionHasBeenStarted)?;
    let record = ctx.data().use_cases.record.find(&record_id).await?;
    let current_agenda_id = ctx
        .data()
        .current_agenda_id
        .get()
        .ok_or(CommandError::AgendaIsNotFound)?;

    // 投票結果のEmbedを送信
    if ctx.data().vote_message_id.get().is_some() {
        let _ = ctx
            .channel_id()
            .send_message(&ctx.discord().http, |c| {
                c.embed(|e| discord_embed::vote_result(e, &record, &current_agenda_id, &status))
            })
            .await;
    } else {
        let _ = ctx
            .send(|r| {
                r.embed(|e| discord_embed::vote_result(e, &record, &current_agenda_id, &status))
            })
            .await;
    }

    let current_agenda_id = AgendaId::new(current_agenda_id);

    // ステータスに応じてRedmineを更新
    match status {
        AgendaStatus::Approved => {
            ctx.data()
                .use_cases
                .agenda
                .approve(&current_agenda_id)
                .await?;
        }
        AgendaStatus::Declined => {
            ctx.data()
                .use_cases
                .agenda
                .decline(&current_agenda_id)
                .await?;
        }
        _ => {}
    };

    // 投票メッセージIDと現在の議題の記録をリセット
    ctx.data().vote_message_id.clear();
    ctx.data().current_agenda_id.clear();

    // 次の議題を選択
    let agendas = {
        let relations = record
            .relations
            .iter()
            .map(|id| AgendaId::new(id.to_owned()))
            .collect_vec();

        ctx.data()
            .use_cases
            .agenda
            .list_new(&relations)
            .await
            .sort_by_id()
    };
    let next_agenda = agendas.first();
    // 次の議題があれば、グローバル変数に
    if let Some(agenda) = next_agenda {
        ctx.data().current_agenda_id.save(agenda.id);
    }
    // 次の議題の存否に応じてEmbedを送信
    let _ = ctx
        .channel_id()
        .send_message(&ctx.discord().http, |c| {
            c.embed(|e| match next_agenda {
                Some(agenda) => {
                    info!("Next Agenda: {}", agenda.id.as_formatted_id());

                    discord_embed::next_agenda_embed(e, &record, agenda)
                }
                None => {
                    info!("No next agenda");

                    discord_embed::no_next_agenda(e, &record)
                }
            })
        })
        .await;

    Ok(())
}
