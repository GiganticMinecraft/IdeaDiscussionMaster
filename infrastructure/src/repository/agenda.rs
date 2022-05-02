use super::RedminePersistenceImpl;
use anyhow::ensure;
use domain::{
    id::IssueId,
    redmine::{Agenda, Note},
    repository::AgendaRepository,
    status::StatusExt,
    MyError,
};
use serde_json::json;
use serenity::async_trait;

#[async_trait]
impl AgendaRepository for RedminePersistenceImpl<Agenda> {
    async fn find(&self, id: IssueId) -> anyhow::Result<Agenda> {
        let res = self.client.get(id).await?;
        ensure!(res.issue.is_idea_ticket(), MyError::TicketIsNotIdea);

        res.try_into()
    }

    async fn change_status(&self, new_agenda: Agenda) -> anyhow::Result<()> {
        let json_value = json!({
          "issue": {
            "status_id": new_agenda.status.id()
          }
        });
        let _ = self.client.post(new_agenda.id, json_value).await?;

        Ok(())
    }

    async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()> {
        let notes = note.content.join("\n");
        let json_value = json!({
          "issue": {
            "notes": notes
          }
        });
        let _ = self.client.post(id, json_value).await?;

        Ok(())
    }
}
