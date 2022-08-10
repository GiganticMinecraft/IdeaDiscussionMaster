use super::RedmineRepositoryImpl;
use crate::UseStatusId;

use c_domain::{id::AgendaId, repository::AgendaRepository, status::AgendaStatus, Agenda};

use anyhow::{anyhow, ensure};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl AgendaRepository for RedmineRepositoryImpl<Agenda> {
    async fn find(&self, id: AgendaId) -> anyhow::Result<Agenda> {
        let res = self.client.get(id.into()).await?;
        ensure!(res.issue.is_idea_ticket(), anyhow!("is not idea ticket"));

        Ok(res.issue.into())
    }

    async fn change_status(&self, id: AgendaId, status: AgendaStatus) -> anyhow::Result<()> {
        let json_value = json!({
          "issue": {
            "status_id": status.id()
          }
        });

        self.client.put(id.into(), &json_value).await.map(|_| ())
    }
}
