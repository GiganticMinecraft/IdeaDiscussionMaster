use crate::redmine::{serde_opt_naive_date, UseStatusId};
use c_domain::redmine::model::Record;

use chrono::NaiveDate;
use derive_new::new;
use serde::Serialize;

#[derive(Serialize, new)]
pub struct CreateRecordParam {
    #[new(value = "18")]
    project_id: u16,
    #[new(value = "34")]
    tracker_id: u16,
    status_id: u16,
    pub subject: String,
    description: String,
    #[serde(with = "serde_opt_naive_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<NaiveDate>,
    #[serde(with = "serde_opt_naive_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<NaiveDate>,
}

impl From<Record> for CreateRecordParam {
    fn from(record: Record) -> Self {
        Self::new(
            record.status.id(),
            record.title,
            record.description,
            record.start_date,
            record.due_date,
        )
    }
}

#[derive(Serialize, new)]
pub struct CreateRecord {
    issue: CreateRecordParam,
}
