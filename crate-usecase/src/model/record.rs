use crate_domain::{id::IssueId, redmine::Record, status::RecordStatus};

use derive_new::new;

#[derive(new, Debug, Clone)]
pub struct RecordDto {
    pub id: IssueId,
    pub title: String,
    pub status: RecordStatus,
    pub relations: Vec<IssueId>,
}

impl From<Record> for RecordDto {
    fn from(record: Record) -> Self {
        Self::new(record.id, record.title, record.status, record.relations)
    }
}
