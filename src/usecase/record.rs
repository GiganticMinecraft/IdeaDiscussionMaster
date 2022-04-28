use super::model::RecordDto;
use crate::domain::{
    id::IssueId,
    repository::RecordRepository,
    status::{record::RecordStatus, StatusExt},
    ticket::{Note, Record},
    MyError,
};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct RecordUseCase<R: RecordRepository> {
    repository: Arc<R>,
}

impl<R: RecordRepository> RecordUseCase<R> {
    pub async fn find_new_one(&self) -> anyhow::Result<RecordDto> {
        self.find(|ticket| ticket.status.is_new())
            .await
            .map(|r| r.into())
    }

    async fn find_by_id_with_record(&self, id: IssueId) -> anyhow::Result<Record> {
        self.find(|ticket| ticket.id == id).await
    }

    pub async fn find_by_id(&self, id: IssueId) -> anyhow::Result<RecordDto> {
        self.find_by_id_with_record(id).await.map(|r| r.into())
    }

    pub async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()> {
        self.repository.add_note(id, note).await
    }

    pub async fn close(&self, id: IssueId) -> anyhow::Result<()> {
        let record = self.find_by_id_with_record(id).await?;
        let new = record.close();

        self.repository.update(new).await
    }

    pub async fn add_relation(&self, id: IssueId, relation: IssueId) -> anyhow::Result<()> {
        self.repository.add_relation(id, relation).await
    }

    // TODO: 切り出す
    #[allow(dead_code)]
    async fn find<P>(&self, f: P) -> anyhow::Result<Record>
    where
        P: FnMut(&&Record) -> bool,
    {
        let list = self.repository.list().await?;

        list.iter()
            .find(f)
            .map(|ticket| ticket.to_owned())
            .ok_or_else(|| MyError::TicketIsNotFound.into())
    }
}
