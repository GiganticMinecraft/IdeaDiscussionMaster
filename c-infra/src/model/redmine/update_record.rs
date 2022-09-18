use crate::UseStatusId;
use c_domain::redmine::model::Record;

use derive_new::new;
use serde::Serialize;

#[derive(Serialize, new)]
pub struct UpdateRecordParam {
    title: String,
    description: String,
    status_id: u16,
}

impl From<Record> for UpdateRecordParam {
    fn from(record: Record) -> Self {
        Self::new(record.title, record.description, record.status.id())
    }
}

#[derive(Serialize, new)]
pub struct UpdateRecord {
    issue: UpdateRecordParam,
}
