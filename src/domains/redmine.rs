use serde::Deserialize;

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
    pub subject: String,
    pub description: String,
    #[serde(default)]
    pub relations: Vec<RedmineIssueRelations>,
}
#[derive(Debug, Deserialize)]
pub struct RedmineIssueResult {
    pub issue: RedmineIssue,
}
