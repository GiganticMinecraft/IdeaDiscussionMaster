use crate::redmine::UseStatusId;
use c_domain::redmine::model::Agenda;

use derive_new::new;
use serde::Serialize;

#[derive(Serialize, new)]
pub struct UpdateAgendaParam {
    subject: String,
    description: String,
    status_id: u16,
}

impl From<Agenda> for UpdateAgendaParam {
    fn from(agenda: Agenda) -> Self {
        Self::new(agenda.title, agenda.description, agenda.status.id())
    }
}

#[derive(Serialize, new)]
pub struct UpdateAgenda {
    issue: UpdateAgendaParam,
}
