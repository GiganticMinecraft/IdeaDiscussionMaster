use super::model::Agenda;
use crate_domain::{
    id::IssueId,
    status::{AgendaStatus, StatusExt},
};
use crate_utils::HashSetExt;

use once_cell::sync::Lazy;
use serenity::model::id::MessageId;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

type Agendas = HashSet<Agenda>;

static AGENDAS: Lazy<Arc<Mutex<Agendas>>> = Lazy::new(|| Arc::new(Mutex::new(HashSet::new())));

pub fn add(agenda: Agenda) -> Agendas {
    let mut set = AGENDAS.lock().unwrap();
    set.insert(agenda);

    set.clone()
}

pub fn list() -> Agendas {
    AGENDAS.lock().unwrap().clone()
}

pub fn find_by_id(id: IssueId) -> Option<Agenda> {
    list().iter().find(|agenda| agenda.id == id).copied()
}

pub fn update_status(id: IssueId, status: AgendaStatus) -> Agendas {
    let agenda = find_by_id(id).unwrap_or_else(|| Agenda::new(id.0));
    let new_agenda = Agenda { status, ..agenda };

    let mut set = AGENDAS.lock().unwrap();
    set.update_or_insert(&agenda, new_agenda);

    set.clone()
}

pub fn update_votes_message_id(id: IssueId, votes_message_id: Option<MessageId>) -> Agendas {
    let agenda = find_by_id(id).unwrap_or_else(|| Agenda::new(id.0));
    let new_agenda = Agenda {
        votes_message_id,
        ..agenda
    };

    let mut set = AGENDAS.lock().unwrap();
    set.update_or_insert(&agenda, new_agenda);

    set.clone()
}

pub fn find_current() -> Option<Agenda> {
    list().iter().find(|agenda| agenda.status.is_new()).copied()
}

pub fn clear() -> Agendas {
    AGENDAS.lock().unwrap().clear();

    list()
}
