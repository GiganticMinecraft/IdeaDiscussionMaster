use crate::domain::{
    id::IssueId,
    repository::RecordRepository,
    status::record::RecordStatus,
    ticket::{Note, Record},
};
use std::sync::Arc;

pub struct RecordUseCase<R: RecordRepository> {
    repository: Arc<R>,
}

impl<R: RecordRepository> RecordUseCase<R> {
    pub async fn find_new_one(&self) -> Option<Record> {
        self.find(|ticket| ticket.status == RecordStatus::New).await
    }

    pub async fn find_by_id(&self, id: IssueId) -> Option<Record> {
        self.find(|ticket| ticket.id == id).await
    }

    pub async fn add_note(&self, id: IssueId, note: Note) {
        self.repository.add_note(id, note).await
    }

    pub async fn close(&self, id: IssueId) {
        let record = self.find_by_id(id).await;

        if let Some(record) = record {
            let new = record.close();
            self.repository.update(new).await;
        }
    }

    // TODO: 切り出す
    #[allow(dead_code)]
    async fn find<P>(&self, f: P) -> Option<Record>
    where
        P: FnMut(&&Record) -> bool,
    {
        self.repository
            .list()
            .await
            .iter()
            .find(f)
            .map(|ticket| ticket.to_owned())
    }
}
