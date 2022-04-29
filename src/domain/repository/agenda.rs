use crate::domain::{
    id::IssueId,
    ticket::{Agenda, Note},
};
use serenity::async_trait;

#[async_trait]
pub trait AgendaRepository {
    async fn find(&self, id: IssueId) -> anyhow::Result<Agenda>;
    async fn change_status(&self, new_agenda: Agenda) -> anyhow::Result<()>;
    async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()>;
}
