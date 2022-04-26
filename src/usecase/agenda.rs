use crate::domain::{
    id::IssueId,
    repository::AgendaRepository,
    ticket::{Agenda, Note},
};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
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
            self.repository.update(new).await;
        }
    }

    pub async fn decline(&self, id: IssueId) {
        let agenda = self.repository.find(id).await;

        if let Some(agenda) = agenda {
            let new = agenda.decline();
            self.repository.update(new).await;
        }
    }

    pub async fn add_note(&self, id: IssueId, note: Note) {
        self.repository.add_note(id, note).await;
    }

    pub async fn add_notes(&self, id: IssueId, notes: Vec<Note>) {
        for note in notes.into_iter() {
            self.add_note(id, note).await;
        }
    }
}
