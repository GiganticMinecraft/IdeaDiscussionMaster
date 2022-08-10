use super::RedmineUrlInterpreter;
use crate::model::redmine::{RedmineIssueResult, RedmineIssuesResult};

use itertools::Itertools;
use std::sync::Arc;
use surf::{Client, Response};

pub struct RedmineClient {
    pub client: Arc<Client>,
    pub url_interpreter: RedmineUrlInterpreter,
}

// TODO: use custom error
impl RedmineClient {
    pub fn new(url: String) -> Self {
        Self {
            client: Arc::new(Client::new()),
            url_interpreter: RedmineUrlInterpreter::new(url),
        }
    }

    pub async fn get(&self, id: u16) -> anyhow::Result<RedmineIssueResult> {
        let url = self.url_interpreter.issue_url(id);

        self.client
            .get(url)
            .query(&[("include", "relations")])
            .unwrap()
            .recv_json::<RedmineIssueResult>()
            .await
            .map_err(|e| e.into_inner())
    }

    pub async fn get_list<T: ToString, U: ToString>(
        &self,
        queries: Vec<(T, U)>,
    ) -> anyhow::Result<RedmineIssuesResult> {
        let url = self.url_interpreter.issues_url();
        let queries = queries
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect_vec();

        self.client
            .get(url)
            .query(&queries)
            .unwrap()
            .recv_json::<RedmineIssuesResult>()
            .await
            .map_err(|e| e.into_inner())
    }

    pub async fn put(&self, id: u16, json: &impl serde::Serialize) -> anyhow::Result<Response> {
        self.put_with_url(self.url_interpreter.issue_url(id), json)
            .await
    }

    pub async fn put_with_url(
        &self,
        url: String,
        json: &impl serde::Serialize,
    ) -> anyhow::Result<Response> {
        self.client
            .put(url)
            .header("Content-Type", "application/json")
            .body_json(json)
            .unwrap()
            .send()
            .await
            .map_err(|e| e.into_inner())
    }

    pub async fn post(&self, id: u16, value: &impl serde::Serialize) -> anyhow::Result<Response> {
        self.put_with_url(self.url_interpreter.issue_url(id), value)
            .await
    }

    pub async fn post_with_url(
        &self,
        url: String,
        value: &impl serde::Serialize,
    ) -> anyhow::Result<Response> {
        self.client
            .post(url)
            .header("Content-Type", "application/json")
            .body_json(&value)
            .unwrap()
            .send()
            .await
            .map_err(|e| e.into_inner())
    }
}
