use super::RedmineRepositoryImpl;
use crate::model::redmine::UpdateAgenda;
use c_domain::{id::AgendaId, repository::AgendaRepository, Agenda};

use anyhow::{anyhow, ensure};
use async_trait::async_trait;

#[async_trait]
impl AgendaRepository for RedmineRepositoryImpl<Agenda> {
    async fn find(&self, id: AgendaId) -> anyhow::Result<Agenda> {
        let res = self.client.get(id.into()).await?;
        ensure!(res.issue.is_idea_ticket(), anyhow!("is not idea ticket"));

        Ok(res.issue.into())
    }

    async fn save(&self, agenda: Agenda) -> anyhow::Result<()> {
        let agenda_id = agenda.id.clone();
        let value = UpdateAgenda::new(agenda.into());
        let _ = self.client.put(agenda_id.into(), &value).await?;

        Ok(())
    }
}
