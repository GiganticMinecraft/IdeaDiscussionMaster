use crate::domain::id::IssueId;
use derive_new::new;

#[derive(new)]
pub struct RecordDto {
    pub id: IssueId,
    pub title: String,
    pub status: RecordStatus,
    pub relations: Vec<IssueId>,
}
