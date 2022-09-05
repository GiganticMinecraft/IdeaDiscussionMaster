use crate::{id::RecordId, status::RecordStatus};

use chrono::NaiveDate;
use derive_new::new;

#[derive(new, PartialEq, Eq, Debug, Default, Clone)]
pub struct Record {
    pub id: RecordId,
    pub title: String,
    pub description: String,
    pub status: RecordStatus,
    pub relations: Vec<RecordId>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
}

impl Record {
    pub fn close(self) -> Self {
        Self {
            status: RecordStatus::Closed,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    #[test_case(Record::close => RecordStatus::Closed; "close")]
    fn change_status(f: fn(Record) -> Record) -> RecordStatus {
        let record = Record::default();
        let record = f(record);

        record.status
    }
}
