use super::DtoExt;
use crate_domain::{id::IssueId, redmine::Record, status::RecordStatus};
use crate_shared::REDMINE_URL;

use anyhow::anyhow;
use chrono::NaiveDate;
use derive_new::new;
use regex::Regex;

#[derive(new, Debug, Clone)]
pub struct RecordDto {
    pub id: IssueId,
    pub title: String,
    pub status: RecordStatus,
    pub relations: Vec<IssueId>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
}

impl RecordDto {
    pub fn discussion_title(&self) -> String {
        Self::title_regex()
            .find(&self.title)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "アイデア会議".to_string())
    }

    pub fn discussion_number(&self) -> anyhow::Result<u16> {
        let cap = Self::title_regex()
            .captures(&self.title)
            .ok_or_else(|| anyhow!("No matches in record title"))?;

        cap[1]
            .parse::<u16>()
            .map_err(|_| anyhow!("Error while parsing record num to u16"))
    }

    fn title_regex() -> Regex {
        Regex::new(r"第([1-9][0-9]*)回アイデア会議").unwrap()
    }
}

impl DtoExt for RecordDto {
    fn url(&self) -> String {
        format!("{}/issues/{}", REDMINE_URL, self.id.0)
    }
}

impl From<Record> for RecordDto {
    fn from(record: Record) -> Self {
        Self::new(
            record.id,
            record.title,
            record.status,
            record.relations,
            record.start_date,
            record.due_date,
        )
    }
}
