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
