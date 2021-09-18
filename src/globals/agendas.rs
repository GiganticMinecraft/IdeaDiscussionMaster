use serenity::{
    model::id::MessageId,
    prelude::{Context, TypeMapKey},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::domains::status::agenda_status::AgendaStatus;

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

impl TypeMapKey for Agendas {
    type Value = Arc<RwLock<HashMap<u16, Agenda>>>;
}

pub async fn read(ctx: &Context) -> HashMap<u16, Agenda> {
    let cached_agendas = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Agendas>()
            .expect("Expected Agendas in TypeMap.")
            .clone()
    };
    let map = cached_agendas.read().await;
    map.to_owned()
}

pub async fn write(ctx: &Context, id: u16, new_agenda: Agenda) {
    let cached_agendas = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Agendas>()
            .expect("Expected Agendas in TypeMap.")
            .clone()
    };
    let mut map = cached_agendas.write().await;
    map.entry(id)
        .and_modify(|agenda| *agenda = new_agenda)
        .or_insert(new_agenda);
}

pub async fn clear(ctx: &Context) {
    let cached_agendas = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Agendas>()
            .expect("Expected Agendas in TypeMap.")
            .clone()
    };
    let mut map = cached_agendas.write().await;
    map.clear();
}
