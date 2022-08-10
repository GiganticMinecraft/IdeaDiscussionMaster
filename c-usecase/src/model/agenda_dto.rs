use c_domain::{status::AgendaStatus, Agenda};

use derive_new::new;

#[derive(new, Debug, Clone, Hash, Eq, PartialEq)]
pub struct AgendaDto {
    pub id: u16,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}

impl From<Agenda> for AgendaDto {
    fn from(agenda: Agenda) -> Self {
        Self::new(
            agenda.id.into(),
            agenda.title,
            agenda.description,
            agenda.status,
        )
    }
}
