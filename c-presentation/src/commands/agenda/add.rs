use crate::{
    commands::{CommandResult, Context},
    shared::{
        ext::{CreateEmbedExt, UseFormattedId},
        CommandError,
    },
};
use c_domain::id::{AgendaId, RecordId};

use crate::shared::discord_embed;
use log::info;

/// 議題を追加します
#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "追加する議題のチケット番号"]
    #[min = 1]
    new_agenda_id: u16,
) -> CommandResult {
    let new_agenda_id = AgendaId::new(new_agenda_id);
    let new_agenda = ctx.data().use_cases.agenda.find_new(&new_agenda_id).await?;
    let record_id = ctx
        .data()
        .record_id
        .get()
        .map(RecordId::new)
        .ok_or(CommandError::DiscussionHasBeenStarted)?;

    // 関連付けをする
    ctx.data()
        .use_cases
        .record
        .add_relation(&record_id, &new_agenda_id)
        .await?;
    info!(
        "Agenda added: agenda_id: {:?}, record_id: {:?}",
        new_agenda_id, record_id
    );
    let _ = ctx
        .send(|c| {
            c.embed(|e| {
                e.custom_default(&record_id)
                    .title("議題を追加しました")
                    .description(format!("追加した議題: {}", new_agenda_id.formatted()))
                    .success_color()
            })
        })
        .await?;

    // 現在進行中の議題がなければ、議題として提示
    if ctx.data().current_agenda_id.get().is_none() {
        info!("Next Agenda: {}", new_agenda_id.formatted());
        ctx.data().current_agenda_id.save(new_agenda.id);
        let _ = ctx
            .channel_id()
            .send_message(&ctx.discord().http, |c| {
                c.embed(|e| discord_embed::next_agenda_embed(e, &record_id, &new_agenda))
            })
            .await?;
    }

    Ok(())
}
