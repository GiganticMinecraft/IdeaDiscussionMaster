use super::model::{AgendaDto, CreateNoteParam};
use c_domain::redmine::{model::id::AgendaId, repository::AgendaRepository};

use anyhow::{ensure, Context as _};
use derive_new::new;
use futures::future;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct AgendaUseCase {
    repo: Arc<dyn AgendaRepository + Sync + Send>,
}

impl AgendaUseCase {
    pub async fn find(&self, id: &AgendaId) -> anyhow::Result<AgendaDto> {
        self.repo
            .find(id)
            .await
            .map(|a| a.into())
            .context("議題の取得に失敗しました")
    }

    pub async fn find_new(&self, id: &AgendaId) -> anyhow::Result<AgendaDto> {
        let issue = self.find(id).await?;
        ensure!(
            issue.status.is_new(),
            "議題のステータスが新規ではありません"
        );

        Ok(issue)
    }

    pub async fn list(&self, agendas: &[AgendaId]) -> Vec<AgendaDto> {
        future::join_all(agendas.iter().map(|id| async move { self.find(id).await }))
            .await
            .into_iter()
            .filter_map(|agenda| agenda.ok())
            .collect()
    }

    pub async fn list_new(&self, agendas: &[AgendaId]) -> Vec<AgendaDto> {
        self.list(agendas)
            .await
            .into_iter()
            .filter(|dto| dto.status.is_new())
            .collect()
    }

    pub async fn init(&self, id: &AgendaId) -> anyhow::Result<()> {
        let agenda = self.repo.find(id).await?;
        let agenda = agenda.init()?;

        self.repo.save(agenda).await
    }

    pub async fn in_progress(&self, id: &AgendaId) -> anyhow::Result<()> {
        let agenda = self.repo.find(id).await?;
        let agenda = agenda.in_progress()?;

        self.repo.save(agenda).await
    }

    pub async fn approve(&self, id: &AgendaId) -> anyhow::Result<()> {
        let agenda = self.repo.find(id).await?;
        let agenda = agenda.approve()?;

        self.repo.save(agenda).await
    }

    pub async fn decline(&self, id: &AgendaId) -> anyhow::Result<()> {
        let agenda = self.repo.find(id).await?;
        let agenda = agenda.decline()?;

        self.repo.save(agenda).await
    }

    pub async fn add_note(&self, id: &AgendaId, param: CreateNoteParam) -> anyhow::Result<()> {
        self.repo.add_note(id, param.into()).await
    }
}
