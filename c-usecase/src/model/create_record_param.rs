use chrono::NaiveDate;
use derive_new::new;

#[derive(new)]
pub struct CreateRecordParam {
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
}
