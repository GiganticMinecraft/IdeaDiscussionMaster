use crate::domain::{id::IssueId, status::RecordStatus};

pub struct Record {
    pub id: IssueId,
    pub title: String,
    pub status: RecordStatus,
    pub relations: Vec<IssueId>,
}

impl Record {
    pub fn new(id: IssueId, title: String, relations: Vec<IssueId>) -> Self {
        Self {
            id,
            title,
            relations,
            status: RecordStatus::New,
        }
    }

    pub fn close(self) -> Self {
        Self {
            status: RecordStatus::Done,
            ..self
        }
    }
}
