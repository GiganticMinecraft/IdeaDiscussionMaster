use super::model::AgendaDto;
use crate::domain::{id::IssueId, repository::AgendaRepository, ticket::Note};
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

    pub async fn accept(&self, id: IssueId) {
        let agenda = self.repository.find(id).await;

        if let Ok(agenda) = agenda {
            let new = agenda.accept();
            self.repository.update(new).await;
        }
    }

    pub async fn decline(&self, id: IssueId) {
        let agenda = self.repository.find(id).await;

        if let Ok(agenda) = agenda {
            let new = agenda.decline();
            self.repository.update(new).await;
        }
    }

    pub async fn add_note(&self, id: IssueId, note: Note) {
        self.repository.add_note(id, note).await;
    }
}
