use std::vec;

use c_domain::{id::RecordId, status::RecordStatus, Record};

pub trait RecordFixture {
    fn new1() -> Record {
        Record::new(
            RecordId::new(1),
            String::new(),
            String::new(),
            RecordStatus::New,
            Vec::new(),
            None,
            None,
        )
    }

    fn closed1() -> Record {
        Record::new(
            RecordId::new(2),
            String::new(),
            String::new(),
            RecordStatus::Closed,
            Vec::new(),
            None,
            None,
        )
    }

    fn closed2() -> Record {
        Record::new(
            RecordId::new(3),
            String::new(),
            String::new(),
            RecordStatus::Closed,
            Vec::new(),
            None,
            None,
        )
    }

    fn closed3() -> Record {
        Record::new(
            RecordId::new(4),
            String::new(),
            String::new(),
            RecordStatus::Closed,
            Vec::new(),
            None,
            None,
        )
    }

    fn all_fixtures() -> Vec<Record> {
        vec![
            Self::new1(),
            Self::closed1(),
            Self::closed2(),
            Self::closed3(),
        ]
    }
}

impl RecordFixture for Record {}
