use super::model::{
    id::{AgendaId, RecordId},
    status::RecordStatus,
    Agenda, Note, Record,
};

use async_trait::async_trait;

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait]
pub trait AgendaRepository {
    async fn find(&self, id: &AgendaId) -> anyhow::Result<Agenda>;
    async fn save(&self, agenda: Agenda) -> anyhow::Result<()>;
    async fn add_note(&self, id: &AgendaId, note: Note) -> anyhow::Result<()>;
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait]
pub trait RecordRepository {
    async fn add(&self, new_record: Record) -> anyhow::Result<Record>;
    async fn find(&self, id: &RecordId) -> anyhow::Result<Record>;
    async fn list(
        &self,
        limit: Option<u16>,
        status: Vec<RecordStatus>,
    ) -> anyhow::Result<Vec<Record>>;
    async fn save(&self, record: Record) -> anyhow::Result<()>;
    async fn add_relation(&self, id: &RecordId, relate_id: &AgendaId) -> anyhow::Result<()>;
    async fn add_note(&self, id: &RecordId, note: Note) -> anyhow::Result<()>;
}
