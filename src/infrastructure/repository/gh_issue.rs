use super::{super::model::github::CreateIssueResponse, GitHubPersistenceImpl};
use crate::{
    domain::{gh_issue::Issue, repository::GitHubIssueRepository},
    util::client::GITHUB_URL,
};
use anyhow::{ensure, Context};
use reqwest::header;
use serde_json::json;
use serenity::async_trait;

#[async_trait]
impl GitHubIssueRepository for GitHubPersistenceImpl<Issue> {
    async fn add(&self, issue: Issue) -> anyhow::Result<String> {
        let content = json!({
            "title": issue.title,
            "body": issue.content,
            "labels": issue.labels
        });

        let res = self
            .client
            .client
            .post(GITHUB_URL)
            .header(header::USER_AGENT, "curl/7.38.0")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/vnd.github.v3+json")
            .header(
                header::AUTHORIZATION,
                format!("token {}", self.client.token),
            )
            .json(&content)
            .send()
            .await
            .context("Error while sending a Http Request")?;

        let status = res.status();
        ensure!(
            status.is_success(),
            "Http Status Error: {} {}",
            status,
            status.canonical_reason().unwrap_or_default()
        );

        let res = res
            .json::<CreateIssueResponse>()
            .await
            .context("Error while deserialize Http Response")?;

        Ok(res.html_url)
    }
}
