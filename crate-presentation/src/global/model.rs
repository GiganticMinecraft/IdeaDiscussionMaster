use crate_domain::{id::IssueId, status::AgendaStatus};
use crate_usecase::model::AgendaDto;

use serenity::model::id::MessageId;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Agenda {
    pub id: IssueId,
    pub status: AgendaStatus,
    pub votes_message_id: Option<MessageId>,
}

impl Agenda {
    pub fn new(id: u16) -> Self {
        Self {
            id: IssueId::new(id),
            status: AgendaStatus::New,
            votes_message_id: None,
        }
    }
}

impl From<AgendaDto> for Agenda {
    fn from(dto: AgendaDto) -> Self {
        Self::new(dto.id.0)
    }
}
