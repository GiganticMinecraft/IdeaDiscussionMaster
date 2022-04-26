use crate::domain::ticket::Record;
use serenity::async_trait;

#[async_trait]
pub trait RecordRepository {
    async fn list(&self) -> Vec<Record>;
    async fn update(&self, record: Record);
}
