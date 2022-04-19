use super::{id::AgendaId, status::AgendaStatus};
use serenity::model::id::MessageId;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Agenda {
    pub id: AgendaId,
    pub status: AgendaStatus,
    pub votes_message_id: Option<MessageId>,
}

impl Agenda {
    pub fn new(id: u16) -> Self {
        Self {
            id: AgendaId::new(id),
            status: AgendaStatus::New,
            votes_message_id: None,
        }
    }
}
