use crate::domain::{id::IssueId, status::AgendaStatus};

#[derive(Debug, Clone)]
pub struct Agenda {
    pub id: IssueId,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}

impl Agenda {
    pub fn new(id: IssueId, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            status: AgendaStatus::New,
        }
    }

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

    pub fn accept(self) -> Self {
        Self {
            status: AgendaStatus::Approved,
            ..self
        }
    }
}
