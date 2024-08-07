use crate::{
    commands::Context,
    shared::{
        discord_embed,
        ext::{SortAgendasExt, UseFormattedId},
        CommandError, VoteChoice,
    },
};
use c_domain::redmine::model::{
    id::{AgendaId, RecordId},
    status::AgendaStatus,
};
use c_usecase::redmine::model::CreateNoteParam;

use itertools::Itertools;
use log::info;
use poise::{
    serenity_prelude::{CacheHttp, CreateEmbed, CreateMessage},
    CreateReply,
};

pub async fn end_votes(ctx: &Context<'_>, choice: VoteChoice) -> anyhow::Result<()> {
    info!("Vote finished: {}", choice);
    let data = ctx.data();
    let agenda_use_case = &data.use_cases.agenda;
    let record_id = data
        .record_id
        .get()
        .map(RecordId::new)
        .ok_or(CommandError::DiscussionHasBeenStarted)?;
    let record = data.use_cases.record.find(&record_id).await?;
    let current_agenda_id = data
        .current_agenda_id
        .get()
        .map(AgendaId::new)
        .ok_or(CommandError::AgendaIsNotFound)?;

    // 投票結果のEmbedを送信
    if data.vote_message_id.get().is_some() {
        let _ = ctx
            .channel_id()
            .send_message(
                &ctx.http(),
                CreateMessage::new().embed(discord_embed::vote_result(
                    CreateEmbed::new(),
                    &record_id,
                    &current_agenda_id,
                    &choice,
                )),
            )
            .await;
    } else {
        let _ = ctx
            .send(CreateReply::default().embed(discord_embed::vote_result(
                CreateEmbed::new(),
                &record_id,
                &current_agenda_id,
                &choice,
            )))
            .await;
    }

    // ステータスに応じてRedmineを更新
    let _ = agenda_use_case
        .add_note(
            &current_agenda_id,
            CreateNoteParam::new(format!("{}", choice)),
        )
        .await;
    match choice.status {
        AgendaStatus::Approved => {
            agenda_use_case.approve(&current_agenda_id).await?;
        }
        AgendaStatus::Declined => {
            agenda_use_case.decline(&current_agenda_id).await?;
        }
        _ => {}
    };

    // 投票メッセージIDと現在の議題の記録をリセット
    data.vote_message_id.clear();
    data.current_agenda_id.clear();

    // 次の議題を選択
    let agendas = {
        let relations = record
            .relations
            .iter()
            .map(|id| AgendaId::new(id.to_owned()))
            .collect_vec();

        agenda_use_case.list_new(&relations).await.sort_by_id()
    };
    let next_agenda = agendas.first();
    // 次の議題があれば、グローバル変数に
    if let Some(agenda) = next_agenda {
        data.current_agenda_id.save(agenda.id);
    }
    // 次の議題の存否に応じてEmbedを送信
    let embed = match next_agenda {
        Some(agenda) => {
            info!("Next Agenda: {}", AgendaId::new(agenda.id).formatted());
            discord_embed::next_agenda_embed(CreateEmbed::new(), &record_id, agenda)
        }
        None => {
            info!("No next agenda");
            discord_embed::no_next_agenda(CreateEmbed::new(), &record_id)
        }
    };
    let _ = ctx
        .channel_id()
        .send_message(&ctx.http(), CreateMessage::new().embed(embed))
        .await;

    Ok(())
}
