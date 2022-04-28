use super::model::AgendaDto;
use crate::domain::{
    id::IssueId,
    repository::AgendaRepository,
    status::{agenda::AgendaStatus, StatusExt},
    ticket::Note,
    MyError,
};
use anyhow::ensure;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct AgendaUseCase<R: AgendaRepository> {
    repository: Arc<R>,
}

impl<R: AgendaRepository> AgendaUseCase<R> {
    pub async fn find(&self, id: IssueId) -> anyhow::Result<AgendaDto> {
        self.repository.find(id).await.map(|a| a.into())
    }

    pub async fn find_new(&self, id: IssueId) -> anyhow::Result<AgendaDto> {
        let issue = self.find(id).await?;
        ensure!(issue.status.is_new(), MyError::TicketIsNotUndoneIdea);

        Ok(issue)
    }

    pub async fn accept(&self, id: IssueId) -> anyhow::Result<()> {
        let agenda = self.repository.find(id).await;

        match agenda {
            Ok(agenda) => {
                let new = agenda.accept();
                self.repository.update(new).await
            }
            Err(e) => Err(e),
        }
    }

    pub async fn decline(&self, id: IssueId) -> anyhow::Result<()> {
        let agenda = self.repository.find(id).await;

        match agenda {
            Ok(agenda) => {
                let new = agenda.decline();
                self.repository.update(new).await
            }
            Err(e) => Err(e),
        }
    }

    pub async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()> {
        self.repository.add_note(id, note).await
    }
}
