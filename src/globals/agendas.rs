use crate::domains::status::AgendaStatus;
use serenity::{
    model::id::MessageId,
    prelude::{Context, TypeMapKey},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone, Copy, Debug)]
pub struct Agenda {
    pub status: AgendaStatus,
    pub votes_message_id: Option<MessageId>,
}

impl Agenda {
    pub fn new(status: AgendaStatus, votes_message_id: Option<MessageId>) -> Self {
        Self {
            status,
            votes_message_id,
        }
    }
}

impl Default for Agenda {
    fn default() -> Self {
        Self::new(AgendaStatus::New, None)
    }
}

pub struct Agendas;

type AgendasType = HashMap<u16, Agenda>;
type AgendasTypeMapKey = Arc<RwLock<AgendasType>>;

impl TypeMapKey for Agendas {
    type Value = AgendasTypeMapKey;
}

async fn get_lock(ctx: &Context) -> AgendasTypeMapKey {
    let data_read = ctx.data.read().await;
    data_read
        .get::<Agendas>()
        .expect("Expected Agendas in TypeMap.")
        .clone()
}

pub async fn read(ctx: &Context) -> AgendasType {
    let lock = get_lock(ctx).await;
    let map = lock.read().await;
    map.to_owned()
}

pub async fn write(ctx: &Context, id: u16, new_agenda: Agenda) {
    let lock = get_lock(ctx).await;
    let mut map = lock.write().await;
    map.entry(id)
        .and_modify(|agenda| *agenda = new_agenda)
        .or_insert(new_agenda);
}

pub async fn update_status(ctx: &Context, id: u16, new_status: AgendaStatus) {
    let map = read(ctx).await;
    let agenda = map
        .get(&id)
        .map_or(Agenda::default(), |agenda| agenda.to_owned());
    write(ctx, id, Agenda::new(new_status, agenda.votes_message_id)).await;
}

pub async fn update_votes_message_id(ctx: &Context, id: u16, new_msg_id: Option<MessageId>) {
    let map = read(ctx).await;
    let agenda = map
        .get(&id)
        .map_or(Agenda::default(), |agenda| agenda.to_owned());
    write(ctx, id, Agenda::new(agenda.status, new_msg_id)).await;
}

pub async fn find_current_agenda(ctx: &Context) -> Option<(u16, Agenda)> {
    let map = read(ctx).await;
    map.iter()
        .find(|agenda| agenda.1.status.is_in_progress())
        .map(|(id, agenda)| (id.to_owned(), agenda.to_owned()))
}

pub async fn find_current_agenda_id(ctx: &Context) -> Option<u16> {
    find_current_agenda(ctx).await.map(|agenda| agenda.0)
}

pub async fn find_votes_message_id(ctx: &Context, id: u16) -> Option<MessageId> {
    let map = read(ctx).await;
    map.get(&id).and_then(|agenda| agenda.votes_message_id)
}

pub async fn clear_all(ctx: &Context) {
    let lock = get_lock(ctx).await;
    let mut map = lock.write().await;
    map.clear();
}
