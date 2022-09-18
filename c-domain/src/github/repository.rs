use super::model::Issue;

use async_trait::async_trait;

#[async_trait]
pub trait GitHubIssueRepository {
    async fn add(&self, issue: Issue) -> anyhow::Result<String>;
}
