use crate::{
    commands::{CommandResult, Context},
    shared::{
        discord_embed,
        ext::{CreateEmbedExt, UseFormattedId},
        CommandError,
    },
};
use c_domain::redmine::model::id::{AgendaId, RecordId};

use log::info;
use poise::{
    serenity_prelude::{CreateEmbed, CreateMessage},
    CreateReply,
};

/// 議題を追加します
#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "追加する議題のチケット番号"]
    #[min = 1]
    new_agenda_id: u16,
) -> CommandResult {
    let _ = ctx.defer().await;
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
        .send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .custom_default(&record_id)
                    .title("議題を追加しました")
                    .description(format!("追加した議題: {}", new_agenda_id.formatted()))
                    .success_color(),
            ),
        )
        .await?;

    // 現在進行中の議題がなければ、議題として提示
    if ctx.data().current_agenda_id.get().is_none() {
        info!("Next Agenda: {}", new_agenda_id.formatted());
        ctx.data().current_agenda_id.save(new_agenda.id);
        let _ = ctx
            .channel_id()
            .send_message(
                &ctx.http(),
                CreateMessage::new().embed(discord_embed::next_agenda_embed(
                    CreateEmbed::new(),
                    &record_id,
                    &new_agenda,
                )),
            )
            .await?;
    }

    Ok(())
}
