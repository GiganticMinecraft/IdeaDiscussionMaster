use super::RedminePersistenceImpl;
use crate_domain::{
    id::IssueId,
    redmine::{Note, Record},
    repository::RecordRepository,
    status::StatusExt,
    MyError,
};

use anyhow::ensure;
use serde_json::json;
use serenity::async_trait;

#[async_trait]
impl RecordRepository for RedminePersistenceImpl<Record> {
    async fn find(&self, id: IssueId) -> anyhow::Result<Record> {
        let res = self.client.get(id).await?;
        ensure!(
            res.issue.is_idea_discussion_record(),
            MyError::TicketIsNotIdea
        );

        res.try_into()
    }

    async fn change_status(&self, new_record: Record) -> anyhow::Result<()> {
        let json_value = json!({
          "issue": {
            "status_id": new_record.status.id()
          }
        });
        let _ = self.client.post(new_record.id, json_value).await?;

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

    async fn add_relation(&self, id: IssueId, relate_id: IssueId) -> anyhow::Result<()> {
        let json_value = json!({
          "relation": {
            "issue_to_id": relate_id.0,
            "relation_type": "relates"
          }
        });
        let _ = self
            .client
            .post_with_url(self.client.issue_relations_url(id), json_value)
            .await?;

        Ok(())
    }
}
