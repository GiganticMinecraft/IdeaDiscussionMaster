use serenity::{prelude::{Context, TypeMapKey}, model::id::ChannelId};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct VoiceChatChannelId;

type LockTypeMapKey = Arc<RwLock<Option<ChannelId>>>;

impl TypeMapKey for VoiceChatChannelId {
    type Value = LockTypeMapKey;
}

async fn get_lock(ctx: &Context) -> LockTypeMapKey {
    let data_read = ctx.data.read().await;
    data_read
        .get::<VoiceChatChannelId>()
        .expect("Expected VoiceChatChannelId in TypeMap.")
        .clone()
}

pub async fn read(ctx: &Context) -> Option<ChannelId> {
    let lock = get_lock(ctx).await;
    let id = lock.read().await;
    id.to_owned()
}

pub async fn write(ctx: &Context, vc_id: Option<ChannelId>) {
    let lock = get_lock(ctx).await;
    let mut id = lock.write().await;
    *id = vc_id;
}

pub async fn clear(ctx: &Context) {
    write(ctx, None).await;
}
