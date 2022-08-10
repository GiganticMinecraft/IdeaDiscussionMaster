use super::{id::AgendaId, status::AgendaStatus};

use derive_new::new;

#[derive(new, PartialEq, Debug, Default, Clone)]
pub struct Agenda {
    pub id: AgendaId,
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

    pub fn approve(self) -> Self {
        Self {
            status: AgendaStatus::Approved,
            ..self
        }
    }

    pub fn decline(self) -> Self {
        Self {
            status: AgendaStatus::Declined,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    #[test_case(Agenda::in_progress => AgendaStatus::InProgress; "in_progress")]
    #[test_case(Agenda::approve => AgendaStatus::Approved; "approve")]
    #[test_case(Agenda::decline => AgendaStatus::Declined; "decline")]
    fn change_status(f: fn(Agenda) -> Agenda) -> AgendaStatus {
        let agenda = Agenda::default();
        let agenda = f(agenda);

        agenda.status
    }
}
