use itertools::Itertools;
use serenity::prelude::{Context, TypeMapKey};
use std::{collections::HashMap, sync::Arc};
use strum::{Display, EnumIter, EnumProperty, EnumString, IntoEnumIterator};
use tokio::sync::RwLock;

pub struct Agendas;

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumProperty, EnumString, PartialEq)]
pub enum AgendaStatus {
    #[strum(ascii_case_insensitive, props(ja = "新規", emoji = "🆕"))]
    New,
    #[strum(
        ascii_case_insensitive,
        props(ja = "承認", emoji = "⭕", is_done = "true")
    )]
    Approved,
    #[strum(
        ascii_case_insensitive,
        props(ja = "却下", emoji = "❌", is_done = "true")
    )]
    Declined,
}

impl AgendaStatus {
    pub fn emoji(self) -> String {
        self.get_str("emoji").unwrap().to_string()
    }

    pub fn ja(self) -> String {
        self.get_str("ja").unwrap().to_string()
    }

    pub fn from(ch: char) -> Option<Self> {
        Self::iter().find(|status| status.emoji() == ch.to_string())
    }

    pub fn done_statuses() -> Vec<Self> {
        Self::iter()
            .filter(|status| status.get_str("is_done").is_some())
            .collect_vec()
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
    map.entry(id)
        .and_modify(|status| *status = new_status)
        .or_insert(new_status);
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
