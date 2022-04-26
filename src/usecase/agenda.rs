use crate::domain::{
    id::IssueId,
    repository::AgendaRepository,
    ticket::{Agenda, Note},
};
use std::sync::Arc;

pub struct AgendaUseCase<R: AgendaRepository> {
    repository: Arc<R>,
}

impl<R: AgendaRepository> AgendaUseCase<R> {
    pub async fn find(&self, id: IssueId) -> Option<Agenda> {
        self.repository.find(id).await
    }

    pub async fn accept(&self, id: IssueId) {
        let agenda = self.repository.find(id).await;

        if let Some(agenda) = agenda {
            let new = agenda.accept();
            self.repository.update(id, new).await;
        }
    }

    pub async fn decline(&self, id: IssueId) {
        let agenda = self.repository.find(id).await;

        if let Some(agenda) = agenda {
            let new = agenda.decline();
            self.repository.update(id, new).await;
        }
    }

    pub async fn add_note(&self, id: IssueId, note: Note) {
        let agenda = self.repository.find(id).await;

        if let Some(agenda) = agenda {
            let new = agenda.add_note(note);
            self.repository.update(id, new).await;
        }
    }
}
