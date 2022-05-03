use crate_domain::{
    error::MyError,
    id::IssueId,
    redmine::{Agenda, Record},
    status::{agenda::AgendaStatus, record::RecordStatus, StatusExt},
};

use itertools::Itertools;
use serde::Deserialize;

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
    pub name: String,
}

impl TryFrom<RedmineIssueStatus> for AgendaStatus {
    type Error = anyhow::Error;
    fn try_from(status: RedmineIssueStatus) -> anyhow::Result<Self> {
        Self::from_id(status.id)
            .ok_or_else(|| MyError::TicketHasUnexpectedStatus(status.id, status.name).into())
    }
}

impl TryFrom<RedmineIssueStatus> for RecordStatus {
    type Error = anyhow::Error;
    fn try_from(status: RedmineIssueStatus) -> anyhow::Result<Self> {
        Self::from_id(status.id)
            .ok_or_else(|| MyError::TicketHasUnexpectedStatus(status.id, status.name).into())
    }
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

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct RedmineIssueResult {
    pub issue: RedmineIssue,
}

impl TryFrom<RedmineIssueResult> for Agenda {
    type Error = anyhow::Error;
    fn try_from(res: RedmineIssueResult) -> anyhow::Result<Self> {
        let issue = res.issue;
        let status = issue.status.try_into()?;

        Ok(Self::new(
            IssueId::new(issue.id),
            issue.subject,
            issue.description,
            status,
        ))
    }
}

impl TryFrom<RedmineIssueResult> for Record {
    type Error = anyhow::Error;
    fn try_from(res: RedmineIssueResult) -> anyhow::Result<Self> {
        let issue = res.issue;
        let relations = issue
            .relations()
            .iter()
            .map(|id| IssueId::new(*id))
            .collect_vec();
        let status = issue.status.try_into()?;

        Ok(Self::new(
            IssueId::new(issue.id),
            issue.subject,
            status,
            relations,
        ))
    }
}
