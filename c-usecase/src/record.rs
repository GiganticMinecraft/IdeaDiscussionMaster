use super::model::{RecordDto, RecordParam};
use c_domain::{
    id::{AgendaId, RecordId},
    repository::RecordRepository,
    status::RecordStatus,
    Record,
};

use anyhow::{ensure, Context as _};
use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct RecordUseCase {
    repo: Arc<dyn RecordRepository>,
}

impl RecordUseCase {
    pub async fn add(&self, param: RecordParam) -> anyhow::Result<RecordDto> {
        let new_record = Record::new(
            RecordId::default(),
            param.title,
            param.description,
            RecordStatus::New,
            vec![],
            param.start_date,
            param.due_date,
        );

        self.repo
            .add(new_record)
            .await
            .map(|record| record.into())
            .context("議事録の作成に失敗しました")
    }

    pub async fn find(&self, id: RecordId) -> anyhow::Result<RecordDto> {
        self.repo
            .find(id)
            .await
            .map(|r| r.into())
            .context("議事録の取得に失敗しました")
    }

    pub async fn find_new(&self, id: RecordId) -> anyhow::Result<RecordDto> {
        let record = self.find(id).await?;
        ensure!(record.status.is_new(),);

        Ok(record)
    }

    pub async fn list(
        &self,
        limit: Option<u16>,
        status: Vec<RecordStatus>,
    ) -> anyhow::Result<Vec<RecordDto>> {
        self.repo
            .list(limit, status)
            .await
            .map(|vec| vec.into_iter().map(|r| r.into()).collect())
            .context("議事録の一覧の取得に失敗しました")
    }

    pub async fn find_latest_closed(&self) -> anyhow::Result<RecordDto> {
        self.list(Some(1), vec![RecordStatus::Closed])
            .await?
            .first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Error!"))
    }

    async fn change_status(&self, id: RecordId, new_status: RecordStatus) -> anyhow::Result<()> {
        let record = self.repo.find(id).await?;

        self.repo.change_status(record.id, new_status).await
    }

    pub async fn close(&self, id: RecordId) -> anyhow::Result<()> {
        self.change_status(id, RecordStatus::Closed).await
    }

    pub async fn add_relation(&self, id: RecordId, relation: AgendaId) -> anyhow::Result<()> {
        self.repo
            .add_relation(id, relation)
            .await
            .context("議事録にチケットを関連付けできませんでした")
    }
}
