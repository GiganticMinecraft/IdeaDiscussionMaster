use crate::{
    id::{AgendaId, RecordId},
    status::{AgendaStatus, RecordStatus},
    Agenda, Record,
};

use async_trait::async_trait;

#[async_trait]
pub trait AgendaRepository {
    async fn find(&self, id: AgendaId) -> anyhow::Result<Agenda>;
    async fn change_status(&self, id: AgendaId, status: AgendaStatus) -> anyhow::Result<()>;
}

#[async_trait]
pub trait RecordRepository {
    async fn add(&self, new_record: Record) -> anyhow::Result<Record>;
    async fn find(&self, id: RecordId) -> anyhow::Result<Record>;
    async fn list(
        &self,
        limit: Option<u16>,
        status: Vec<RecordStatus>,
    ) -> anyhow::Result<Vec<Record>>;
    async fn change_status(&self, id: RecordId, status: RecordStatus) -> anyhow::Result<()>;
    async fn add_relation(&self, id: RecordId, relate_id: AgendaId) -> anyhow::Result<()>;
}
