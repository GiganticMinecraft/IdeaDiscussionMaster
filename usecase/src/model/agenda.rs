use derive_new::new;
use domain::{id::IssueId, redmine::Agenda, status::agenda::AgendaStatus};

#[derive(new)]
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
