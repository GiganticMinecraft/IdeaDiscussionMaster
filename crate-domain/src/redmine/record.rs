use crate::{id::IssueId, status::record::RecordStatus};

use chrono::NaiveDate;
use derive_new::new;

#[derive(Clone, new, Debug, Default)]
pub struct Record {
    pub id: IssueId,
    pub title: String,
    pub description: String,
    pub status: RecordStatus,
    pub relations: Vec<IssueId>,
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

    #[test]
    fn succeeded_in_close() {
        assert_eq!(Record::default().close().status, RecordStatus::Closed);
    }
}
