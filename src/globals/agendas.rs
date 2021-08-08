use serenity::prelude::TypeMapKey;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct Agendas;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AgendaStatus {
    New,
    Approved,
    Declined,
}

impl TypeMapKey for Agendas {
    type Value = Arc<RwLock<HashMap<u16, AgendaStatus>>>;
}
