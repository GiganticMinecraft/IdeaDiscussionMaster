use serenity::prelude::{Context, TypeMapKey};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::domains::status::agenda_status::AgendaStatus;

#[deprecated]
pub struct Agendas;

#[allow(deprecated)]
impl TypeMapKey for Agendas {
    type Value = Arc<RwLock<HashMap<u16, AgendaStatus>>>;
}

#[allow(deprecated)]
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

#[allow(deprecated)]
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

#[allow(deprecated)]
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
