use super::model::{RecordDto, RecordParam};
use crate_domain::{
    error::MyError,
    id::IssueId,
    redmine::{Note, Record},
    repository::{RecordRepository, RepositoryModuleExt},
    status::{RecordStatus, StatusExt},
};

use anyhow::ensure;
use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct RecordUseCase<R: RepositoryModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoryModuleExt> RecordUseCase<R> {
    pub async fn add(&self, param: RecordParam) -> anyhow::Result<RecordDto> {
        let new_record = Record::new(
            IssueId::default(),
            param.title,
            param.description,
            RecordStatus::default(),
            vec![],
            param.start_date,
            param.due_date,
        );

        self.repositories
            .record_repository()
            .add(new_record)
            .await
            .map(|record| record.into())
    }

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

    pub async fn list(
        &self,
        limit: Option<u16>,
        status: Vec<RecordStatus>,
    ) -> anyhow::Result<Vec<RecordDto>> {
        self.repositories
            .record_repository()
            .list(limit, status)
            .await
            .map(|vec| vec.into_iter().map(|r| r.into()).collect())
    }

    pub async fn find_latest_closed(&self) -> anyhow::Result<RecordDto> {
        self.list(Some(1), vec![RecordStatus::Closed])
            .await?
            .first()
            .cloned()
            .ok_or_else(|| MyError::TicketIsNotFound.into())
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
