use crate::{
    domain::{id::IssueId, MyError},
    util::{client::REDMINE_URL, Env},
};
use anyhow::{anyhow, Context};
use reqwest::{header, Client, Response, StatusCode};
use std::sync::Arc;

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

    pub async fn get(&self, id: IssueId) -> anyhow::Result<Response> {
        let url = self.generate_url(id);
        let res = self
            .client
            .get(url)
            .query(&[("include", "relations")])
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
        let url = self.generate_url(id);
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

    fn generate_url(&self, id: IssueId) -> String {
        format!("{}/issues/{}.json?{}", REDMINE_URL, id.0, self.token)
    }

    async fn map_by_http_status(res: Response) -> anyhow::Result<Response> {
        let status = res.status();

        if status.is_success() {
            Ok(res)
        } else {
            match status {
                StatusCode::NOT_FOUND | StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED => {
                    Err(MyError::TicketIsNotFound.into())
                }
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