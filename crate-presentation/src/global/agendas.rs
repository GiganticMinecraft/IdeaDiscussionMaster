use super::model::Agenda;
use crate_domain::{
    id::IssueId,
    status::{AgendaStatus, StatusExt},
};
use crate_shared::HashSetExt;

use itertools::Itertools;
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

pub fn grouped_list() -> Vec<(AgendaStatus, Vec<Agenda>)> {
    // 議題をすべて取得し、ステータスでソート
    // ここでソートしないと、そのままの順番でグルーピングされるので、同じステータスの別グループができる
    let agendas = list()
        .into_iter()
        .sorted_by_cached_key(|agenda| agenda.status)
        .collect_vec();

    // 議題をステータスでグルーピング
    // https://stackoverflow.com/questions/47885478/how-to-use-itertools-group-by-iterator-method-without-a-for-loop
    agendas
        .iter()
        .group_by(|agenda| agenda.status)
        .into_iter()
        .map(|(status, group)| (status, group.cloned().collect()))
        .collect()
}

pub fn find_by_id(id: IssueId) -> Option<Agenda> {
    list().iter().find(|agenda| agenda.id == id).copied()
}

pub fn in_progress(id: IssueId) -> Agendas {
    // すでに進行中の議題があれば、ステータスを新規に変更
    if let Some(current) = find_current() {
        update_status(current.id, AgendaStatus::New);
    }

    update_status(id, AgendaStatus::InProgress)
}

fn update_status(id: IssueId, status: AgendaStatus) -> Agendas {
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

pub fn find_next() -> Option<IssueId> {
    list()
        .iter()
        .find(|agenda| agenda.status.is_new())
        .map(|agenda| agenda.id)
}

pub fn clear() -> Agendas {
    AGENDAS.lock().unwrap().clear();

    list()
}
