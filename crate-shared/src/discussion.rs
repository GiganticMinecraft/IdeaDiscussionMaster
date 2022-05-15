use crate_domain::error::MyError;

use anyhow::anyhow;
use serenity::{
    cache::Cache,
    model::{
        id::{ChannelId, GuildId, UserId},
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

pub async fn find_vc_by_user_id(
    cache: &Arc<Cache>,
    guild_id: &GuildId,
    user_id: &UserId,
) -> anyhow::Result<ChannelId> {
    get_voice_states(cache, guild_id)
        .await?
        .get(user_id)
        .and_then(|state| state.channel_id)
        .ok_or_else(|| MyError::IsNotJoinedInVC.into())
}
