use crate::domain::{id::IssueId, status::AgendaStatus, ticket::Note};

#[derive(Debug, Clone)]
pub struct Agenda {
    pub id: IssueId,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
    pub notes: Vec<Note>,
}

impl Agenda {
    pub fn new(id: IssueId, title: String, description: String, notes: Vec<Note>) -> Self {
        Self {
            id,
            title,
            description,
            status: AgendaStatus::New,
            notes,
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

    pub fn add_note(self, note: Note) -> Self {
        let mut notes = self.notes;
        notes.push(note);

        Self { notes, ..self }
    }
}
