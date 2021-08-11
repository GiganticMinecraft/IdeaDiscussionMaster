use serenity::prelude::{Context, TypeMapKey};
use std::sync::{atomic::{AtomicU64, Ordering}, Arc};

pub struct VoiceChatChannelId;

impl TypeMapKey for VoiceChatChannelId {
    type Value = Arc<AtomicU64>;
}

pub async fn read(ctx: &Context) -> u64 {
    let cached_current_agenda_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<VoiceChatChannelId>()
            .expect("Expected VoiceChatChannelId in TypeMap.")
            .clone()
    };
    cached_current_agenda_id.load(Ordering::Relaxed)
}

pub async fn write(ctx: &Context, new_agenda_id: u64) -> u64 {
    let cached_current_agenda_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<VoiceChatChannelId>()
            .expect("Expected VoiceChatChannelId in TypeMap.")
            .clone()
    };
    cached_current_agenda_id.store(new_agenda_id, Ordering::Relaxed);
    new_agenda_id
}

pub async fn clear(ctx: &Context) {
    write(ctx, 0).await;
}
