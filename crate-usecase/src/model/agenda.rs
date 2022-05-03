use crate_domain::{id::IssueId, redmine::Agenda, status::agenda::AgendaStatus};

use derive_new::new;

#[derive(new, Debug)]
pub struct AgendaDto {
    pub id: IssueId,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}

impl From<Agenda> for AgendaDto {
    fn from(agenda: Agenda) -> Self {
        Self::new(agenda.id, agenda.title, agenda.description, agenda.status)
    }
}
