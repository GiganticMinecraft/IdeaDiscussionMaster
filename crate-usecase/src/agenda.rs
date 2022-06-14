use super::model::AgendaDto;
use crate_domain::repository::RepositoryModuleExt;
use crate_domain::{
    error::MyError, id::IssueId, redmine::Note, repository::AgendaRepository, status::StatusExt,
};

use anyhow::{ensure, Context as _};
use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct AgendaUseCase<R: RepositoryModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoryModuleExt> AgendaUseCase<R> {
    pub async fn find(&self, id: IssueId) -> anyhow::Result<AgendaDto> {
        self.repositories
            .agenda_repository()
            .find(id)
            .await
            .map(|a| a.into())
            .context("議題の取得に失敗しました")
    }

    pub async fn find_new(&self, id: IssueId) -> anyhow::Result<AgendaDto> {
        let issue = self.find(id).await?;
        ensure!(issue.status.is_new(), MyError::TicketIsNotUndoneIdea);

        Ok(issue)
    }

    pub async fn approve(&self, id: IssueId) -> anyhow::Result<()> {
        let repo = self.repositories.agenda_repository();
        let agenda = repo.find(id).await;

        match agenda {
            Ok(agenda) => {
                let new = agenda.approve();
                repo.change_status(new).await
            }
            Err(e) => Err(e),
        }
        .context("ステータスの変更に失敗しました")
    }

    pub async fn decline(&self, id: IssueId) -> anyhow::Result<()> {
        let repo = self.repositories.agenda_repository();
        let agenda = repo.find(id).await;

        match agenda {
            Ok(agenda) => {
                let new = agenda.decline();
                repo.change_status(new).await
            }
            Err(e) => Err(e),
        }
        .context("ステータスの変更に失敗しました")
    }

    pub async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()> {
        self.repositories
            .agenda_repository()
            .add_note(id, note)
            .await
            .context("注釈の追加に失敗しました")
    }
}
