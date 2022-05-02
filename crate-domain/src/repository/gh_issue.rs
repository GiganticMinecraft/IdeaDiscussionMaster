use crate::github::Issue;
use serenity::async_trait;

#[async_trait]
pub trait GitHubIssueRepository {
    async fn add(&self, issue: Issue) -> anyhow::Result<String>;
}
