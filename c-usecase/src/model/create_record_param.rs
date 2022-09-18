use c_domain::{id::RecordId, status::RecordStatus, Record};

use chrono::NaiveDate;
use derive_new::new;

#[derive(new)]
pub struct CreateRecordParam {
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
}

impl From<CreateRecordParam> for Record {
    fn from(param: CreateRecordParam) -> Self {
        Record::new(
            RecordId::default(),
            param.title,
            param.description,
            RecordStatus::default(),
            Vec::default(),
            param.start_date,
            param.due_date,
        )
    }
}
