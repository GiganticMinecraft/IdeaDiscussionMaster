use super::RedmineRepositoryImpl;
use crate::{
    model::redmine::{CreateRecord, UpdateRecord},
    UseStatusId,
};
use c_domain::{
    id::{AgendaId, RecordId},
    repository::RecordRepository,
    status::RecordStatus,
    Record,
};

use anyhow::{anyhow, ensure};
use async_trait::async_trait;
use itertools::Itertools;
use serde_json::json;
use std::collections::HashMap;

#[async_trait]
impl RecordRepository for RedmineRepositoryImpl<Record> {
    async fn add(&self, record: Record) -> anyhow::Result<Record> {
        let new_record = CreateRecord::new(record.clone().into());
        // FIXME: postの結果を読み込んでRecordを返す（getし直さない）
        self.client
            .post_with_url(self.client.url_interpreter.issues_url(), &new_record)
            .await?;

        Ok(self
            .list(None, vec![RecordStatus::New])
            .await?
            .into_iter()
            .find(|r| r.title == record.title)
            .unwrap())
    }

    async fn find(&self, id: RecordId) -> anyhow::Result<Record> {
        let res = self.client.get(id.into()).await?;
        ensure!(
            res.issue.is_idea_discussion_record(),
            anyhow!("チケットはアイデア会議の議事録ではありません")
        );

        Ok(res.issue.into())
    }

    async fn list(
        &self,
        limit: Option<u16>,
        status: Vec<RecordStatus>,
    ) -> anyhow::Result<Vec<Record>> {
        let limit = limit.unwrap_or(20).to_string();
        // TODO: ProjectIdやTrackerIdをまとめておく
        let mut queries = HashMap::new();
        queries.insert("project_id", "18");
        queries.insert("tracker_id", "34");
        queries.insert("limit", &limit);
        let status = status
            .iter()
            .map(|status| status.id().to_string())
            .join(",");
        if !status.is_empty() {
            queries.insert("status_id", &status);
        }
        let res = self.client.get_list(queries).await?;
        ensure!(
            res.issues
                .iter()
                .all(|issue| issue.is_idea_discussion_record()),
            anyhow!("this is not record")
        );

        Ok(res
            .issues
            .into_iter()
            .filter_map(|issue| issue.try_into().ok())
            .collect_vec())
    }

    async fn save(&self, record: Record) -> anyhow::Result<()> {
        let record_id = record.id.clone();
        let value = UpdateRecord::new(record.into());
        let _ = self.client.put(record_id.into(), &value).await?;

        Ok(())
    }

    async fn add_relation(&self, id: RecordId, relate_id: AgendaId) -> anyhow::Result<()> {
        let relate_id: u16 = relate_id.into();
        let value = json!({
          "relation": {
            "issue_to_id": relate_id,
            "relation_type": "relates"
          }
        });
        let _ = self
            .client
            .post_with_url(
                self.client.url_interpreter.issue_relations_url(id.into()),
                &value,
            )
            .await?;

        Ok(())
    }
}
