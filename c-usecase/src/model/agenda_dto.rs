use c_domain::redmine::model::{status::AgendaStatus, Agenda};
use crate_shared::REDMINE_URL;

use derive_new::new;

#[derive(new, Debug, Clone, Hash, Eq, PartialEq)]
pub struct AgendaDto {
    pub id: u16,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}

impl AgendaDto {
    pub fn url(&self) -> String {
        format!("{}/issues/{}", REDMINE_URL, self.id)
    }
}

impl From<Agenda> for AgendaDto {
    fn from(agenda: Agenda) -> Self {
        Self::new(agenda.id.0, agenda.title, agenda.description, agenda.status)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success_into() {
        let agenda = Agenda::default();
        let dto: AgendaDto = agenda.clone().into();
        let expected_dto = AgendaDto {
            id: agenda.id.0,
            title: agenda.title,
            description: agenda.description,
            status: agenda.status,
        };

        assert_eq!(dto, expected_dto);
    }
}
