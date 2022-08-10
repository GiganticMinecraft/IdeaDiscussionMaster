use super::model::AgendaDto;
use c_domain::{id::AgendaId, repository::AgendaRepository, status::AgendaStatus};

use anyhow::{ensure, Context as _};
use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct AgendaUseCase {
    repo: Arc<dyn AgendaRepository>,
}

impl AgendaUseCase {
    pub async fn find(&self, id: AgendaId) -> anyhow::Result<AgendaDto> {
        self.repo
            .find(id)
            .await
            .map(|a| a.into())
            .context("議題の取得に失敗しました")
    }

    pub async fn find_new(&self, id: AgendaId) -> anyhow::Result<AgendaDto> {
        let issue = self.find(id).await?;
        ensure!(issue.status.is_new());

        Ok(issue)
    }

    async fn change_status(&self, id: AgendaId, new_status: AgendaStatus) -> anyhow::Result<()> {
        let agenda = self.repo.find(id).await?;

        self.repo.change_status(agenda.id, new_status).await
    }

    pub async fn approve(&self, id: AgendaId) -> anyhow::Result<()> {
        self.change_status(id, AgendaStatus::Approved).await
    }

    pub async fn decline(&self, id: AgendaId) -> anyhow::Result<()> {
        self.change_status(id, AgendaStatus::Declined).await
    }
}
