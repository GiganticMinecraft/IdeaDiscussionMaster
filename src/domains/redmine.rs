use serde::Deserialize;
use itertools::Itertools;

use crate::domains::status::AgendaStatus;

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
    pub name: String,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineIssueRelations {
    pub issue_id: u16,
    pub issue_to_id: u16,
    pub relation_type: String,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
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

impl RedmineIssue {
    pub fn is_idea_ticket(&self) -> bool {
        self.project.name == "アイデア提案用プロジェクト"
            && self.tracker.name == "アイデア提案"
            && !AgendaStatus::done_statuses()
                .iter()
                .map(|status| status.ja())
                .contains(&self.status.name)
    }

    pub fn is_idea_discussion_record(&self) -> bool {
        self.project.name == "アイデア会議議事録"
            && self.tracker.name == "アイデア会議"
            && self.status.name == AgendaStatus::New.ja()
    }
}

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineIssueResult {
    pub issue: RedmineIssue,
}
