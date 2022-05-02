use super::model::RecordDto;
use crate::infrastructure::module::RepositoryModuleExt;
use anyhow::ensure;
use derive_new::new;
use domain::{
    id::IssueId, redmine::Note, repository::RecordRepository, status::StatusExt, MyError,
};
use std::sync::Arc;

#[derive(new, Clone)]
pub struct RecordUseCase<R: RepositoryModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoryModuleExt> RecordUseCase<R> {
    pub async fn find(&self, id: IssueId) -> anyhow::Result<RecordDto> {
        self.repositories
            .record_repository()
            .find(id)
            .await
            .map(|r| r.into())
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
        self.repositories
            .record_repository()
            .add_note(id, note)
            .await
    }

    pub async fn close(&self, id: IssueId) -> anyhow::Result<()> {
        let repo = self.repositories.record_repository();
        let record = repo.find(id).await?;
        let new = record.close();

        repo.change_status(new).await
    }

    pub async fn add_relation(&self, id: IssueId, relation: IssueId) -> anyhow::Result<()> {
        self.repositories
            .record_repository()
            .add_relation(id, relation)
            .await
    }
}
