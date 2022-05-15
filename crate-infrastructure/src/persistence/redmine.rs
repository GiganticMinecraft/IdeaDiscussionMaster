use super::super::model::redmine::{RedmineIssueResult, RedmineIssuesResult};
use crate_domain::{error::MyError, id::IssueId};
use crate_shared::{Env, REDMINE_URL};

use anyhow::{anyhow, Context};
use itertools::Itertools;
use reqwest::{header, Client, Response, StatusCode};
use std::sync::Arc;

#[derive(Clone)]
pub struct Redmine {
    pub client: Arc<Client>,
    pub token: String,
}

const REQWEST_ERROR_CONTEXT: &str = "Error while sending a Http Request";

impl Redmine {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::new()),
            token: format!("key={}", Env::new().redmine_api_key),
        }
    }

    pub async fn get(&self, id: IssueId) -> anyhow::Result<RedmineIssueResult> {
        let url = self.issue_url(id);
        let res = self
            .client
            .get(url)
            .query(&[("include", "relations")])
            .send()
            .await
            .context(REQWEST_ERROR_CONTEXT)?;
        let res = Self::map_by_http_status(res).await?;

        res.json::<RedmineIssueResult>()
            .await
            .context("Error while deserializing json")
    }

    pub async fn get_as_list<T: ToString, U: ToString>(
        &self,
        queries: Vec<(T, U)>,
    ) -> anyhow::Result<RedmineIssuesResult> {
        let url = self.issues_url();
        let queries = queries
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect_vec();
        let res = self
            .client
            .get(url)
            .query(&queries)
            .send()
            .await
            .context(REQWEST_ERROR_CONTEXT)?;
        let res = Self::map_by_http_status(res).await?;

        res.json::<RedmineIssuesResult>()
            .await
            .context("Error while deserializing json")
    }

    pub async fn put(
        &self,
        id: IssueId,
        json_value: serde_json::Value,
    ) -> anyhow::Result<Response> {
        self.put_with_url(self.issue_url(id), json_value).await
    }

    pub async fn put_with_url(
        &self,
        url: String,
        json_value: serde_json::Value,
    ) -> anyhow::Result<Response> {
        let res = self
            .client
            .put(url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&json_value)
            .send()
            .await
            .context(REQWEST_ERROR_CONTEXT)?;

        Self::map_by_http_status(res).await
    }

    pub async fn post(
        &self,
        id: IssueId,
        json_value: serde_json::Value,
    ) -> anyhow::Result<Response> {
        self.put_with_url(self.issue_url(id), json_value).await
    }

    pub async fn post_with_url(
        &self,
        url: String,
        json_value: serde_json::Value,
    ) -> anyhow::Result<Response> {
        let res = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&json_value)
            .send()
            .await
            .context(REQWEST_ERROR_CONTEXT)?;

        Self::map_by_http_status(res).await
    }

    fn issue_url(&self, id: IssueId) -> String {
        format!("{}/issues/{}.json?{}", REDMINE_URL, id.0, self.token)
    }

    pub fn issues_url(&self) -> String {
        format!("{}/issues.json?{}", REDMINE_URL, self.token)
    }

    pub fn issue_relations_url(&self, id: IssueId) -> String {
        format!(
            "{}/issues/{}/relations.json?{}",
            REDMINE_URL, id.0, self.token
        )
    }

    async fn map_by_http_status(res: Response) -> anyhow::Result<Response> {
        let status = res.status();

        if status.is_success() {
            Ok(res)
        } else {
            match status {
                StatusCode::NOT_FOUND => Err(MyError::TicketIsNotFound.into()),
                _ => Err(anyhow!(
                    "Unexpected Http Status: {} {}",
                    status.as_str(),
                    status.canonical_reason().unwrap_or_default()
                )),
            }
        }
    }
}

impl Default for Redmine {
    fn default() -> Self {
        Self::new()
    }
}
