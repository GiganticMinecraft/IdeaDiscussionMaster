use super::model::AgendaDto;
use crate_domain::repository::RepositoryModuleExt;
use crate_domain::{
    error::MyError, id::IssueId, redmine::Note, repository::AgendaRepository, status::StatusExt,
};

use anyhow::ensure;
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
    }

    pub async fn find_new(&self, id: IssueId) -> anyhow::Result<AgendaDto> {
        let issue = self.find(id).await?;
        ensure!(issue.status.is_new(), MyError::TicketIsNotUndoneIdea);

        Ok(issue)
    }

    // FIXME: change to approve
    pub async fn accept(&self, id: IssueId) -> anyhow::Result<()> {
        let repo = self.repositories.agenda_repository();
        let agenda = repo.find(id).await;

        match agenda {
            Ok(agenda) => {
                let new = agenda.accept();
                repo.change_status(new).await
            }
            Err(e) => Err(e),
        }
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
    }

    pub async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()> {
        self.repositories
            .agenda_repository()
            .add_note(id, note)
            .await
    }
}
