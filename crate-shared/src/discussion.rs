use anyhow::anyhow;
use serenity::{
    cache::Cache,
    model::{
        id::{GuildId, UserId},
        voice::VoiceState,
    },
};
use std::{collections::HashMap, sync::Arc};

pub async fn get_voice_states(
    cache: &Arc<Cache>,
    guild_id: &GuildId,
) -> anyhow::Result<HashMap<UserId, VoiceState>> {
    cache
        .guild(guild_id)
        .await
        .ok_or_else(|| anyhow!("guildが見つかりませんでした。（guild_id: {}）", guild_id))
        .map(|guild| guild.voice_states)
}
