use super::DtoExt;
use crate_domain::{id::IssueId, redmine::Agenda, status::agenda::AgendaStatus};
use crate_shared::REDMINE_URL;

use derive_new::new;

#[derive(new, Debug, Clone, Hash, Eq, PartialEq)]
pub struct AgendaDto {
    pub id: IssueId,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}

impl DtoExt for AgendaDto {
    fn url(&self) -> String {
        format!("{}/issues/{}", REDMINE_URL, self.id.0)
    }
}

impl From<Agenda> for AgendaDto {
    fn from(agenda: Agenda) -> Self {
        Self::new(agenda.id, agenda.title, agenda.description, agenda.status)
    }
}
