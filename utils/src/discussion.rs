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
) -> HashMap<UserId, VoiceState> {
    let guild_id = if let Some(id) = guild_id {
        id
    } else {
        println!("guild_idが見つかりませんでした。");

        return HashMap::default();
    };
    let guild = if let Some(guild) = ctx.cache.guild(guild_id).await {
        guild
    } else {
        println!("guildが見つかりませんでした。（guild_id: {}）", guild_id);

        return HashMap::default();
    };
    guild.voice_states
}
