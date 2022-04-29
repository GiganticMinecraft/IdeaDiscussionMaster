use crate::domain::{
    id::IssueId,
    ticket::{Note, Record},
};
use serenity::async_trait;

#[async_trait]
pub trait RecordRepository {
    async fn find(&self, id: IssueId) -> anyhow::Result<Record>;
    async fn change_status(&self, new_record: Record) -> anyhow::Result<()>;
    async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()>;
    async fn add_relation(&self, id: IssueId, relate_id: IssueId) -> anyhow::Result<()>;
}
