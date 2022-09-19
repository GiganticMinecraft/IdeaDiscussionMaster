use crate::github::{model::CreateIssueResponse, GitHubClient};
use c_domain::github::{model::Issue, repository::GitHubIssueRepository};
use crate_shared::GITHUB_URL;

use anyhow::{anyhow, ensure, Context as _};
use async_trait::async_trait;
use derive_new::new;
use serde_json::json;
use std::marker::PhantomData;

pub struct GitHubRepositoryImpl<T> {
    pub client: GitHubClient,
    _marker: PhantomData<T>,
}

impl<T> GitHubRepositoryImpl<T> {
    pub async fn new() -> Self {
        Self {
            client: GitHubClient::new().await,
            _marker: PhantomData::default(),
        }
    }
}

#[async_trait]
impl GitHubIssueRepository for GitHubRepositoryImpl<Issue> {
    async fn add(&self, issue: Issue) -> anyhow::Result<String> {
        let content = json!({
            "title": issue.title,
            "body": issue.content,
            "labels": issue.labels
        });

        let mut res = self
            .client
            .client
            .post(GITHUB_URL)
            .header("User-Agent", "curl/7.83.0")
            .header("Content-Type", "application/json")
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", self.client.token.clone())
            .body_json(&content)
            .unwrap()
            .send()
            .await
            .map_err(|e| e.into_inner())
            .context("Error while sending a Http Request")?;

        let status = res.status();
        ensure!(
            status.is_success(),
            "Http Status Error: {} {}",
            status,
            status.canonical_reason()
        );

        let res = res
            .body_json::<CreateIssueResponse>()
            .await
            .map_err(|e| e.into_inner())
            .context("Error while deserialize Http Response")?;

        Ok(res.html_url)
    }
}
