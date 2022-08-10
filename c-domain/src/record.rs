use crate::{id::RecordId, status::RecordStatus};

use chrono::NaiveDate;
use derive_new::new;

#[derive(new, PartialEq, Debug, Default, Clone)]
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

    #[test]
    fn create() {
        let expected_record = Record {
            id: RecordId::new(0),
            title: String::default(),
            description: String::default(),
            status: RecordStatus::New,
            relations: Vec::default(),
            start_date: None,
            due_date: None,
        };

        assert_eq!(
            Record::new(
                expected_record.id.clone(),
                expected_record.title.clone(),
                expected_record.description.clone(),
                expected_record.status.clone(),
                expected_record.relations.clone(),
                expected_record.start_date,
                expected_record.due_date
            ),
            expected_record
        )
    }

    #[test]
    fn create_default() {
        assert_eq!(
            Record::default(),
            Record {
                id: RecordId::new(0),
                title: String::default(),
                description: String::default(),
                status: RecordStatus::New,
                relations: Vec::default(),
                start_date: None,
                due_date: None
            }
        )
    }

    #[test_case(Record::close => RecordStatus::Closed; "close")]
    fn change_status(f: fn(Record) -> Record) -> RecordStatus {
        let record = Record::default();
        let record = f(record);

        record.status
    }
}
