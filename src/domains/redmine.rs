use crate::utils;
use serde::Deserialize;
use std::{collections::HashMap, error};

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
pub struct RedmineIssueRelations {
    pub issue_id: u16,
    pub issue_to_id: u16,
    pub relation_type: String,
}
#[derive(Debug, Deserialize)]
pub struct RedmineIssue {
    pub id: u16,
    pub project: RedmineProject,
    pub tracker: RedmineIssueTracker,
    pub status: RedmineIssueStatus,
    pub relations: Vec<RedmineIssueRelations>,
}
#[derive(Debug, Deserialize)]
struct RedmineIssueResult {
    issue: RedmineIssue,
}

const REDMINE_URL: &str = "https://redmine.seichi.click/";

pub async fn fetch_issue(issue_id: u16, query: Option<HashMap<&str, &str>>) -> Result<RedmineIssue, Box<(dyn error::Error)>> {
    let response = utils::fetch(
        format!("{}/issues/{}.json", REDMINE_URL, issue_id),
        query,
    )
    .await?;
    utils::deserialize::<RedmineIssueResult>(response)
        .await
        .map(|result| result.issue)
}

pub async fn fetch_record_issue(issue_id: u16) -> Result<RedmineIssue, Box<(dyn error::Error)>> {
    let mut query = HashMap::new();
    query.insert("include", "relations");
    fetch_issue(issue_id, Some(query)).await
}
