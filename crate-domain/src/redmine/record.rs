use crate::{id::IssueId, status::record::RecordStatus};
use derive_new::new;

#[derive(Clone, new)]
pub struct Record {
    pub id: IssueId,
    pub title: String,
    pub status: RecordStatus,
    pub relations: Vec<IssueId>,
}

impl Record {
    pub fn close(self) -> Self {
        Self {
            status: RecordStatus::Closed,
            ..self
        }
    }
}