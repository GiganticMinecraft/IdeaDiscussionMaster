use crate::{id::IssueId, status::agenda::AgendaStatus};

use derive_new::new;

#[derive(Debug, Clone, new, Default)]
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

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(Agenda::default().in_progress() => AgendaStatus::InProgress; "in_progress")]
    #[test_case(Agenda::default().approve() => AgendaStatus::Approved; "approved")]
    #[test_case(Agenda::default().decline() => AgendaStatus::Declined; "declined")]
    fn succeed_in_change_status(agenda: Agenda) -> AgendaStatus {
        agenda.status
    }
}
