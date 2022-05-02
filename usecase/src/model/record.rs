use derive_new::new;
use domain::{id::IssueId, redmine::Record, status::RecordStatus};

#[derive(new)]
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
