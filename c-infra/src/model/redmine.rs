use serde::Deserialize;

mod issue;
pub use issue::RedmineIssue;

mod create_record;
pub use create_record::CreateRecord;

mod update_agenda;
pub use update_agenda::UpdateAgenda;

mod update_record;
pub use update_record::UpdateRecord;

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineProject {
    pub name: String,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineIssueTracker {
    pub name: String,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineIssueStatus {
    pub id: u16,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineIssueRelations {
    pub issue_id: u16,
    pub issue_to_id: u16,
    pub relation_type: String,
}

/// `GET /issues/[id]`で返ってくる形
///
/// `/issues`で返ってくるのは[RedmineIssuesResult](RedmineIssuesResult)
#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineIssueResult {
    pub issue: RedmineIssue,
}

/// `GET /issues`で返ってくる形
///
/// `/issues/[id]`で返ってくるのは[RedmineIssueResult](RedmineIssueResult)
#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineIssuesResult {
    pub issues: Vec<RedmineIssue>,
}
