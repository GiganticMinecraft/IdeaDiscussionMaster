use chrono::NaiveDate;
use derive_new::new;

#[derive(new)]
pub struct RecordParam {
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
}
