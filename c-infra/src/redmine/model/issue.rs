use crate::redmine::{
    model::{RedmineIssueRelations, RedmineIssueStatus, RedmineIssueTracker, RedmineProject},
    serde_opt_naive_date, UseStatusId,
};
use c_domain::redmine::model::{
    id::{AgendaId, RecordId},
    status::{AgendaStatus, RecordStatus},
    Agenda, Record,
};

use chrono::NaiveDate;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
pub struct RedmineIssue {
    pub id: u16,
    pub project: RedmineProject,
    pub tracker: RedmineIssueTracker,
    pub status: RedmineIssueStatus,
    pub subject: String,
    pub description: String,
    #[serde(default)]
    #[serde(with = "serde_opt_naive_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    #[serde(default)]
    #[serde(with = "serde_opt_naive_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(default)]
    pub relations: Vec<RedmineIssueRelations>,
}

impl RedmineIssue {
    pub fn is_idea_ticket(&self) -> bool {
        self.project.name == "アイデア提案用プロジェクト" && self.tracker.name == "アイデア提案"
    }

    pub fn is_idea_discussion_record(&self) -> bool {
        self.project.name == "アイデア会議議事録" && self.tracker.name == "アイデア会議"
    }

    pub fn relations(&self) -> Vec<u16> {
        self.relations
            .iter()
            .filter(|rel| rel.relation_type == "relates")
            .flat_map(|rel| vec![rel.issue_id, rel.issue_to_id])
            .collect_vec()
    }
}

impl From<RedmineIssue> for Agenda {
    fn from(issue: RedmineIssue) -> Self {
        let status = AgendaStatus::from_id(issue.status.id);

        Self::new(
            AgendaId::new(issue.id),
            issue.subject,
            issue.description,
            status,
        )
    }
}

impl From<RedmineIssue> for Record {
    fn from(issue: RedmineIssue) -> Self {
        let relations = issue
            .relations()
            .iter()
            .map(|id| RecordId::new(*id))
            .collect_vec();
        let status = RecordStatus::from_id(issue.status.id);

        Self::new(
            RecordId::new(issue.id),
            issue.subject,
            issue.description,
            status,
            relations,
            issue.start_date,
            issue.due_date,
        )
    }
}
