use super::model::RecordDto;
use crate::domain::{
    id::IssueId, repository::RecordRepository, status::StatusExt, ticket::Note, MyError,
};
use anyhow::ensure;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct RecordUseCase<R: RecordRepository> {
    repository: Arc<R>,
}

impl<R: RecordRepository> RecordUseCase<R> {
    pub async fn find(&self, id: IssueId) -> anyhow::Result<RecordDto> {
        self.repository.find(id).await.map(|r| r.into())
    }

    pub async fn find_new(&self, id: IssueId) -> anyhow::Result<RecordDto> {
        let record = self.find(id).await?;
        ensure!(
            record.status.is_new(),
            MyError::TicketIsNotUndoneIdeaDiscussionRecord
        );

        Ok(record)
    }

    pub async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()> {
        self.repository.add_note(id, note).await
    }

    pub async fn close(&self, id: IssueId) -> anyhow::Result<()> {
        let record = self.repository.find(id).await?;
        let new = record.close();

        self.repository.update(new).await
    }

    pub async fn add_relation(&self, id: IssueId, relation: IssueId) -> anyhow::Result<()> {
        self.repository.add_relation(id, relation).await
    }
}
