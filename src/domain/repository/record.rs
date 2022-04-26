use crate::domain::{
    id::IssueId,
    ticket::{Note, Record},
};
use serenity::async_trait;

#[async_trait]
pub trait RecordRepository {
    async fn list(&self) -> Vec<Record>;
    async fn update(&self, record: Record);
    async fn add_note(&self, id: IssueId, note: Note);
}
