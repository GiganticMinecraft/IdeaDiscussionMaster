use crate::domain::{
    id::IssueId,
    ticket::{Note, Record},
};
use serenity::async_trait;

#[async_trait]
pub trait RecordRepository {
    async fn list(&self) -> Vec<Record>;
    async fn update(&self, new_record: Record);
    async fn add_note(&self, id: IssueId, note: Note);
    async fn add_relation(&self, id: IssueId, relate_id: IssueId);
}
