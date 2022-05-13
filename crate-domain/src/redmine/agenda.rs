use crate::{id::IssueId, status::agenda::AgendaStatus};
use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct Agenda {
    pub id: IssueId,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}

impl Agenda {
    pub fn in_progress(self) -> Self {
        Self {
            status: AgendaStatus::InProgress,
            ..self
        }
    }

    pub fn decline(self) -> Self {
        Self {
            status: AgendaStatus::Declined,
            ..self
        }
    }

    pub fn approve(self) -> Self {
        Self {
            status: AgendaStatus::Approved,
            ..self
        }
    }
}
