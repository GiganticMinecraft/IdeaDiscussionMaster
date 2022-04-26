use crate::domain::id::IssueId;
use derive_new::new;

#[derive(new)]
pub struct AgendaDto {
    id: IssueId,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}
