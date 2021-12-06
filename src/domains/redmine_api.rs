use crate::domains::{custom_error, redmine, RedmineClient};

pub const REDMINE_URL: &str = "https://redmine.seichi.click";

pub struct RedmineApi {
    client: RedmineClient,
}

impl RedmineApi {
    pub fn new(client: RedmineClient) -> Self {
        RedmineApi { client }
    }

    pub async fn fetch_issue(
        &self,
        issue_id: u16,
    ) -> Result<redmine::RedmineIssue, custom_error::DiscussionError> {
        self.client.fetch_issue(issue_id).await
    }

    pub async fn fetch_issue_with_relations(
        &self,
        issue_id: u16,
    ) -> Result<redmine::RedmineIssue, custom_error::DiscussionError> {
        self.client.fetch_issue_with_relations(issue_id).await
    }

    pub async fn update_issue_status(
        &self,
        issue_id: u16,
        status_id: u16,
    ) -> Result<reqwest::Response, custom_error::DiscussionError> {
        self.client.update_issue_status(issue_id, status_id).await
    }

    pub async fn add_comments(
        &self,
        issue_id: u16,
        comments: Vec<String>,
    ) -> Result<reqwest::Response, custom_error::DiscussionError> {
        self.client.add_comments(issue_id, comments).await
    }

    pub async fn add_relation(
        &self,
        record_id: u16,
        issue_id: u16,
    ) -> Result<reqwest::Response, custom_error::DiscussionError> {
        self.client.add_relation(record_id, issue_id).await
    }
}
