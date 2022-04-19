use super::status::AgendaStatus;
use serenity::model::id::MessageId;

#[derive(Debug, Clone, Copy)]
pub struct Agenda {
    pub id: u16,
    pub status: AgendaStatus,
    pub votes_message_id: Option<MessageId>,
}
