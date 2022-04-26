use crate::domain::{id::IssueId, ticket::Agenda};
use serenity::async_trait;

#[async_trait]
pub trait AgendaRepository {
    async fn find(&self, id: IssueId) -> Option<Agenda>;
    async fn update(&self, id: IssueId, new_agenda: Agenda);
}
