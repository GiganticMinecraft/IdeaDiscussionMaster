use serenity::prelude::{Context, TypeMapKey};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RecordId;

type LockTypeMapKey = Arc<RwLock<Option<u16>>>;

impl TypeMapKey for RecordId {
    type Value = LockTypeMapKey;
}

async fn get_lock(ctx: &Context) -> LockTypeMapKey {
    let data_read = ctx.data.read().await;
    data_read
        .get::<RecordId>()
        .expect("Expected Agendas in TypeMap.")
        .clone()
}

pub async fn read(ctx: &Context) -> Option<u16> {
    let lock = get_lock(ctx).await;
    let id = lock.read().await;
    id.to_owned()
}

pub async fn write(ctx: &Context, new_agenda_id: Option<u16>) {
    let lock = get_lock(ctx).await;
    let mut id = lock.write().await;
    *id = new_agenda_id;
}

pub async fn clear(ctx: &Context) {
    write(ctx, None).await;
}
