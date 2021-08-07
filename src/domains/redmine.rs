use crate::utils;
use serde::Deserialize;
use std::error;

#[derive(Debug, Deserialize)]
pub struct RedmineProject {
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct RedmineIssueTracker {
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct RedmineIssueStatus {
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct RedmineIssue {
    pub id: u16,
    pub project: RedmineProject,
    pub tracker: RedmineIssueTracker,
    pub status: RedmineIssueStatus,
}
#[derive(Debug, Deserialize)]
struct RedmineIssueResult {
    issue: RedmineIssue,
}

const REDMINE_URL: &str = "https://redmine.seichi.click/";

pub async fn fetch_issue(issue_id: u16) -> Result<RedmineIssue, Box<(dyn error::Error)>> {
    let response = utils::fetch(format!("{}/issues/{}.json", REDMINE_URL, issue_id), None).await?;
    utils::deserialize::<RedmineIssueResult>(response)
        .await
        .map(|result| result.issue)
}
