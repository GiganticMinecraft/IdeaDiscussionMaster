use anyhow::anyhow;
use serenity::{
    model::{
        id::{GuildId, UserId},
        voice::VoiceState,
    },
    prelude::Context,
};
use std::collections::HashMap;

// TODO: return Result

// TODO: なおす

// pub async fn go_to_next_agenda(ctx: &Context) -> Option<u16> {
//     let cached_agendas = agendas::read(ctx).await;
//     let agenda_id = cached_agendas
//         .iter()
//         .find(|(_, agenda)| agenda.status.is_new())
//         .map(|(id, _)| id.to_owned());

//     if agenda_id.is_some() {
//         agendas::update_status(ctx, agenda_id.unwrap(), AgendaStatus::InProgress).await;
//     }

//     agenda_id
// }

pub async fn fetch_voice_states(
    ctx: &Context,
    guild_id: Option<GuildId>,
) -> anyhow::Result<HashMap<UserId, VoiceState>> {
    let guild_id = guild_id.ok_or_else(|| anyhow!("guild_idが見つかりませんでした。"))?;

    ctx.cache
        .guild(guild_id)
        .await
        .ok_or_else(|| anyhow!("guildが見つかりませんでした。（guild_id: {}）", guild_id))
        .map(|guild| guild.voice_states)
}
