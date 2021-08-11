use serenity::prelude::{Context, TypeMapKey};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct Agendas;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AgendaStatus {
    New,
    Approved,
    Declined,
}

impl AgendaStatus {
    pub fn emoji(self) -> char {
        match self {
            AgendaStatus::New => '🆕',
            AgendaStatus::Approved => '⭕',
            AgendaStatus::Declined => '❌',
        }
    }

    pub fn ja(self) -> String {
        match self {
            AgendaStatus::New => "新規",
            AgendaStatus::Approved => "承認",
            AgendaStatus::Declined => "却下"
        }.to_string()
    }

    pub fn from(ch: char) -> Option<AgendaStatus> {
        match ch {
            '🆕' => Some(AgendaStatus::New),
            '⭕' => Some(AgendaStatus::Approved),
            '❌' => Some(AgendaStatus::Declined),
            _ => None
        }
    }
}

impl TypeMapKey for Agendas {
    type Value = Arc<RwLock<HashMap<u16, AgendaStatus>>>;
}

pub async fn read(ctx: &Context) -> HashMap<u16, AgendaStatus> {
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

pub async fn write(ctx: &Context, id: u16, new_status: AgendaStatus) -> HashMap<u16, AgendaStatus> {
    let cached_agendas = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Agendas>()
            .expect("Expected Agendas in TypeMap.")
            .clone()
    };
    let mut map = cached_agendas.write().await;
    map.entry(id).and_modify(|status| *status = new_status).or_insert(new_status);
    map.to_owned()
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
