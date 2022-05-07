use crate_domain::{id::IssueId, redmine::Record, status::RecordStatus};
use crate_shared::REDMINE_URL;

use chrono::NaiveDate;
use derive_new::new;

#[derive(new, Debug, Clone)]
pub struct RecordDto {
    pub id: IssueId,
    pub title: String,
    pub status: RecordStatus,
    pub relations: Vec<IssueId>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
}

impl RecordDto {
    pub fn url(&self) -> String {
        format!("{}/issues/{}", REDMINE_URL, self.id.0)
    }
}

impl From<Record> for RecordDto {
    fn from(record: Record) -> Self {
        Self::new(
            record.id,
            record.title,
            record.status,
            record.relations,
            record.start_date,
            record.due_date,
        )
    }
}
