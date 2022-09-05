use c_domain::{
    id::{AgendaId, MessageId},
    status::AgendaStatus,
};
use c_usecase::model::AgendaDto;

use derive_new::new;

#[derive(new, PartialEq, Eq, Hash, Clone)]
pub struct GlobalAgenda {
    pub id: AgendaId,
    pub status: AgendaStatus,
    #[new(default)]
    pub votes_message_id: Option<MessageId>,
}

impl GlobalAgenda {
    pub fn set_votes_message_id(self, id: MessageId) -> Self {
        Self {
            votes_message_id: Some(id),
            ..self
        }
    }

    pub fn clear_votes_message_id(self) -> Self {
        Self {
            votes_message_id: None,
            ..self
        }
    }
}

impl From<AgendaDto> for GlobalAgenda {
    fn from(dto: AgendaDto) -> Self {
        Self::new(AgendaId::new(dto.id), dto.status)
    }
}
