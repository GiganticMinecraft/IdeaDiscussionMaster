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
