use std::vec;

use c_domain::{id::AgendaId, status::AgendaStatus, Agenda};

pub trait AgendaFixture {
    fn new1() -> Agenda {
        Agenda::new(
            AgendaId::new(1),
            String::new(),
            String::new(),
            AgendaStatus::New,
        )
    }

    fn in_progress() -> Agenda {
        Agenda::new(
            AgendaId::new(2),
            String::new(),
            String::new(),
            AgendaStatus::InProgress,
        )
    }

    fn approved() -> Agenda {
        Agenda::new(
            AgendaId::new(3),
            String::new(),
            String::new(),
            AgendaStatus::Approved,
        )
    }

    fn declined() -> Agenda {
        Agenda::new(
            AgendaId::new(4),
            String::new(),
            String::new(),
            AgendaStatus::Declined,
        )
    }

    fn all_fixtures() -> Vec<Agenda> {
        vec![
            Self::new1(),
            Self::in_progress(),
            Self::approved(),
            Self::declined(),
        ]
    }
}

impl AgendaFixture for Agenda {}
