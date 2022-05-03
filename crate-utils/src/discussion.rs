use crate_domain::error::MyError;

use anyhow::anyhow;
use serenity::{
    cache::Cache,
    model::id::{ChannelId, GuildId, UserId},
};
use std::sync::Arc;

pub async fn find_vc_by_user_id(
    cache: &Arc<Cache>,
    guild_id: &GuildId,
    user_id: &UserId,
) -> anyhow::Result<ChannelId> {
    let guild = cache
        .guild(guild_id)
        .await
        .ok_or_else(|| anyhow!("guildが見つかりませんでした。（guild_id: {}）", guild_id))?;

    guild
        .voice_states
        .get(user_id)
        .and_then(|state| state.channel_id)
        .ok_or_else(|| MyError::IsNotJoinedInVC.into())
}

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
