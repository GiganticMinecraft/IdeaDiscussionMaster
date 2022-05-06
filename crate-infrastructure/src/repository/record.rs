use super::RedminePersistenceImpl;
use crate_domain::{
    error::MyError,
    id::IssueId,
    redmine::{Note, Record},
    repository::RecordRepository,
    status::{RecordStatus, StatusExt},
};

use anyhow::ensure;
use itertools::Itertools;
use serde_json::json;
use serenity::async_trait;

#[async_trait]
impl RecordRepository for RedminePersistenceImpl<Record> {
    async fn find(&self, id: IssueId) -> anyhow::Result<Record> {
        let res = self.client.get(id).await?;
        ensure!(
            res.issue.is_idea_discussion_record(),
            MyError::TicketIsNotIdeaDiscussionRecord
        );

        res.issue.try_into()
    }

    async fn list(&self, limit: Option<u16>) -> anyhow::Result<Vec<Record>> {
        let status = RecordStatus::all()
            .iter()
            .map(|status| status.id().to_string())
            .join(",");
        let limit = limit.unwrap_or(20).to_string();
        let queries = vec![
            ("project_id", "1"),
            ("tracker_id", "34"),
            ("status_id", &status),
            ("sort", "category:created_on"),
            ("limit", &limit),
        ];
        let res = self.client.get_as_list(queries).await?;
        ensure!(
            res.issues
                .iter()
                .all(|issue| issue.is_idea_discussion_record()),
            MyError::TicketIsNotIdeaDiscussionRecord
        );

        Ok(res
            .issues
            .into_iter()
            .filter_map(|issue| issue.try_into().ok())
            .collect_vec())
    }

    async fn change_status(&self, new_record: Record) -> anyhow::Result<()> {
        let json_value = json!({
          "issue": {
            "status_id": new_record.status.id()
          }
        });
        let _ = self.client.put(new_record.id, json_value).await?;

        Ok(())
    }

    async fn add_note(&self, id: IssueId, note: Note) -> anyhow::Result<()> {
        let notes = note.content.join("\n");
        let json_value = json!({
          "issue": {
            "notes": notes
          }
        });
        let _ = self.client.put(id, json_value).await?;

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
