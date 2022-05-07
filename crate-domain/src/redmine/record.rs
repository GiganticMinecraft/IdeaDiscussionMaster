use crate::{id::IssueId, status::record::RecordStatus};

use chrono::NaiveDate;
use derive_new::new;

#[derive(Clone, new, Debug)]
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
