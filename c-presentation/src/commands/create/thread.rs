use super::shared::parse_string_as_agenda_ids;
use crate::{
    commands::{CommandResult, Context},
    shared::{
        discord_embed,
        ext::{SortAgendasExt, UseFormattedId},
    },
};
use c_domain::redmine::model::id::{AgendaId, RecordId};

use itertools::Itertools;
use log::{debug, info};
use poise::serenity_prelude::{AutoArchiveDuration, CreateEmbed, CreateMessage, CreateThread};

/// 承認されたアイデアについて追加議論を行うためのスレッドを作成します
#[poise::command(slash_command)]
pub async fn thread(
    ctx: Context<'_>,
    #[description = "スレッドを作成する議事録の番号"]
    #[min = 1]
    record_issue_number: u16,
    #[description = "スレッドを作成する議題のチケット番号群(半角スペース区切り)"]
    idea_issue_numbers: String,
) -> CommandResult {
    let _ = ctx.defer().await;
    let record_id = RecordId::new(record_issue_number);
    let record = ctx.data().use_cases.record.find(&record_id).await?;
    info!("record_id: {}", record_id.formatted());
    let idea_ids = parse_string_as_agenda_ids(Some(idea_issue_numbers))?;
    let agendas = ctx
        .data()
        .use_cases
        .agenda
        .list(
            &record
                .relations
                .clone()
                .into_iter()
                .map(AgendaId::new)
                .filter(|id| idea_ids.contains(id))
                .collect_vec(),
        )
        .await
        .into_iter()
        .filter(|dto| dto.status.is_approved())
        .collect_vec()
        .sort_by_id();
    info!(
        "selected approved ideas: {}",
        agendas
            .iter()
            .map(|dto| AgendaId::new(dto.id).formatted())
            .join(", ")
    );

    ctx.say(format!("議題{}件のスレッドを作成します", agendas.len()))
        .await?;

    info!("Start to create threads per each idea");
    for agenda in agendas.iter() {
        let formatted_agenda_id = AgendaId::new(agenda.id).formatted();
        debug!("Create a thread for {}", formatted_agenda_id);

        let msg = ctx
            .channel_id()
            .say(
                &ctx.http(),
                format!("{}についてのスレッドを作成しました", formatted_agenda_id),
            )
            .await
            .unwrap();
        let thread = ctx
            .channel_id()
            .create_thread_from_message(
                &ctx.http(),
                msg.id,
                CreateThread::new(format!(
                    "{}: {}",
                    record.discussion_title(),
                    formatted_agenda_id
                ))
                // 24時間でアーカイブされる
                .auto_archive_duration(AutoArchiveDuration::OneDay),
            )
            .await?;
        let _ = thread
            .send_message(
                &ctx.http(),
                CreateMessage::new().content(
                    format!(
                        "このスレッドは、{}にて承認されたアイデア{}について個別に議論を行うためのものです。",
                        record.discussion_title(),
                        formatted_agenda_id
                    )
                ).embed(
                    discord_embed::next_agenda_embed(CreateEmbed::new(), &record_id, agenda)
                        .title(format!("このスレッドで議論を行う議題は{}です", formatted_agenda_id))
                )
                )
            .await?;
    }

    Ok(())
}
