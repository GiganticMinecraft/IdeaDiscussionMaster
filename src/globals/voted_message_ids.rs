use serenity::prelude::{Context, TypeMapKey};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[deprecated]
pub struct VotedMessageIds;

#[allow(deprecated)]
impl TypeMapKey for VotedMessageIds {
    type Value = Arc<RwLock<HashMap<u16, u64>>>;
}

#[allow(deprecated)]
pub async fn read(ctx: &Context) -> HashMap<u16, u64> {
    let cached_current_agenda_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<VotedMessageIds>()
            .expect("Expected VotedMessageIds in TypeMap.")
            .clone()
    };
    let map = cached_current_agenda_id.read().await;
    map.to_owned()
}

#[allow(deprecated)]
pub async fn write(ctx: &Context, agenda_id: u16, voted_message_id: u64) -> HashMap<u16, u64> {
    let cached_voted_message_ids = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<VotedMessageIds>()
            .expect("Expected VotedMessageIds in TypeMap.")
            .clone()
    };
    let mut map = cached_voted_message_ids.write().await;
    map.entry(agenda_id)
        .and_modify(|id| *id = voted_message_id)
        .or_insert(voted_message_id);
    map.to_owned()
}

#[allow(deprecated)]
pub async fn clear(ctx: &Context) {
    let cached_voted_message_ids = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<VotedMessageIds>()
            .expect("Expected Agendas in TypeMap.")
            .clone()
    };
    let mut map = cached_voted_message_ids.write().await;
    map.clear();
}
