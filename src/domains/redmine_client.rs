use mockall::automock;
use reqwest::Client;
use std::collections::HashMap;

use crate::domains::{custom_error, redmine, redmine_api};

pub struct RedmineClient {
    client: Client,
}

impl Default for RedmineClient {
    fn default() -> Self {
        RedmineClient::new()
    }
}

#[automock]
impl RedmineClient {
    pub fn new() -> Self {
        RedmineClient {
            client: Client::new(),
        }
    }

    pub async fn fetch_issue(
        &self,
        issue_id: &u16,
    ) -> Result<redmine::RedmineIssue, custom_error::Error> {
        Ok(fetch(&self.client, issue_id, None)
            .await?
            .json::<redmine::RedmineIssueResult>()
            .await?
            .issue)
    }

    pub async fn fetch_issue_with_relations(
        &self,
        issue_id: &u16,
    ) -> Result<redmine::RedmineIssue, custom_error::Error> {
        let mut query = HashMap::new();
        query.insert("include", "relations");

        Ok(fetch(&self.client, issue_id, Some(query))
            .await?
            .json::<redmine::RedmineIssueResult>()
            .await?
            .issue)
    }
}

async fn fetch(
    client: &Client,
    issue_id: &u16,
    query: Option<HashMap<&str, &str>>,
) -> Result<reqwest::Response, custom_error::Error> {
    let url = format!("{}/issues/{}.json", redmine_api::REDMINE_URL, issue_id);
    let response = client
        .get(url)
        .query(&query.unwrap_or_default())
        .send()
        .await?;

    Ok(response)
}
