use crate::domain::{
    id::IssueId,
    ticket::{Agenda, Note},
};
use serenity::async_trait;

#[async_trait]
pub trait AgendaRepository {
    async fn find(&self, id: IssueId) -> anyhow::Result<Agenda>;
    async fn update(&self, new_agenda: Agenda);
    async fn add_note(&self, id: IssueId, note: Note);
}
