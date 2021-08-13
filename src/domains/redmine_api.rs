use reqwest::Client;
use std::collections::HashMap;

use crate::domains::{redmine, custom_error};

pub const REDMINE_URL: &str = "https://redmine.seichi.click";

pub struct RedmineApi {
    client: Client,
}

impl RedmineApi {
    pub fn new(client: Client) -> Self {
        RedmineApi { client }
    }

    pub async fn fetch_issue(&self, issue_id: &u16) -> Result<redmine::RedmineIssue, custom_error::Error> {
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
    let url = format!("{}/issues/{}.json", REDMINE_URL, issue_id);
    let response = client
        .get(url)
        .query(&query.unwrap_or_default())
        .send()
        .await?;

    Ok(response)
}
