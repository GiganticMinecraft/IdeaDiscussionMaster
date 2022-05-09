use crate::{
    id::IssueId,
    redmine::{Note, Record},
    status::RecordStatus,
};
use serenity::async_trait;

#[async_trait]
pub trait RecordRepository {
    async fn add(&self, new_record: Record) -> anyhow::Result<Record>;
    async fn find(&self, id: IssueId) -> anyhow::Result<Record>;
    async fn list(
        &self,
        limit: Option<u16>,
        status: Vec<RecordStatus>,
    ) -> anyhow::Result<Vec<Record>>;
    async fn change_status(&self, new_record: Record) -> anyhow::Result<()>;
    async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()>;
    async fn add_relation(&self, id: IssueId, relate_id: IssueId) -> anyhow::Result<()>;
}
