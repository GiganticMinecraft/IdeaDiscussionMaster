use super::model::AgendaDto;
use c_domain::{id::AgendaId, repository::AgendaRepository};

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

    pub async fn in_progress(&self, id: AgendaId) -> anyhow::Result<()> {
        let agenda = self.repo.find(id).await?;
        let agenda = agenda.in_progress();

        self.repo.save(agenda).await
    }

    pub async fn approve(&self, id: AgendaId) -> anyhow::Result<()> {
        let agenda = self.repo.find(id).await?;
        let agenda = agenda.approve();

        self.repo.save(agenda).await
    }

    pub async fn decline(&self, id: AgendaId) -> anyhow::Result<()> {
        let agenda = self.repo.find(id).await?;
        let agenda = agenda.decline();

        self.repo.save(agenda).await
    }
}
