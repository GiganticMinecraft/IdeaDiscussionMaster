use mockall::automock;
use reqwest::{header, Client};
use serde_json::json;
use std::{collections::HashMap, env};

use crate::domains::{custom_error, redmine, redmine_api};

pub struct RedmineClient {
    reqwest_client: Client,
    api_key: String,
}

#[automock]
impl RedmineClient {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        RedmineClient {
            reqwest_client: Client::new(),
            api_key: env::var("REDMINE_KEY").unwrap_or_default(),
        }
    }

    pub async fn fetch_issue(
        &self,
        issue_id: u16,
    ) -> Result<redmine::RedmineIssue, custom_error::DiscussionError> {
        Ok(fetch(&self.reqwest_client, issue_id, None)
            .await?
            .json::<redmine::RedmineIssueResult>()
            .await?
            .issue)
    }

    pub async fn fetch_issue_with_relations(
        &self,
        issue_id: u16,
    ) -> Result<redmine::RedmineIssue, custom_error::DiscussionError> {
        let mut query = HashMap::new();
        query.insert("include", "relations");

        Ok(fetch(&self.reqwest_client, issue_id, Some(query))
            .await?
            .json::<redmine::RedmineIssueResult>()
            .await?
            .issue)
    }

    pub async fn update_issue_status(
        &self,
        issue_id: u16,
        status_id: u16,
    ) -> Result<reqwest::Response, custom_error::DiscussionError> {
        let json_value = json!({
          "issue": {
            "status_id": status_id
          }
        });

        update_issue(self, issue_id, json_value).await
    }

    pub async fn add_comments(
        &self,
        issue_id: u16,
        comments: Vec<String>,
    ) -> Result<reqwest::Response, custom_error::DiscussionError> {
        let comments = comments.join("\n");
        let json_value = json!({
          "issue": {
            "notes": comments
          }
        });

        update_issue(self, issue_id, json_value).await
    }

    pub async fn add_relation(
        &self,
        record_id: u16,
        issue_id: u16,
    ) -> Result<reqwest::Response, custom_error::DiscussionError> {
        let url = format!(
            "{}/issues/{}/relations.json?key={}",
            redmine_api::REDMINE_URL,
            record_id,
            self.api_key
        );
        let json_value = json!({
          "relation": {
            "issue_to_id": issue_id,
            "relation_type": "relates"
          }
        });
        let response = self
            .reqwest_client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&json_value)
            .send()
            .await?;

        Ok(response)
    }
}

async fn fetch(
    client: &Client,
    issue_id: u16,
    query: Option<HashMap<&str, &str>>,
) -> Result<reqwest::Response, custom_error::DiscussionError> {
    let url = format!("{}/issues/{}.json", redmine_api::REDMINE_URL, issue_id);
    let response = client
        .get(url)
        .query(&query.unwrap_or_default())
        .send()
        .await?;

    Ok(response)
}

async fn update_issue(
    client: &RedmineClient,
    issue_id: u16,
    json_value: serde_json::Value,
) -> Result<reqwest::Response, custom_error::DiscussionError> {
    let url = format!(
        "{}/issues/{}.json?key={}",
        redmine_api::REDMINE_URL,
        issue_id,
        client.api_key
    );
    let response = client
        .reqwest_client
        .put(url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&json_value)
        .send()
        .await?;

    Ok(response)
}
