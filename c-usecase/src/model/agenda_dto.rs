use c_domain::{status::AgendaStatus, Agenda};

use derive_new::new;

#[derive(new, Debug, Clone, Hash, Eq, PartialEq)]
pub struct AgendaDto {
    pub id: u16,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}

impl From<Agenda> for AgendaDto {
    fn from(agenda: Agenda) -> Self {
        Self::new(
            agenda.id.into(),
            agenda.title,
            agenda.description,
            agenda.status,
        )
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
            id: agenda.id.into(),
            title: agenda.title,
            description: agenda.description,
            status: agenda.status,
        };

        assert_eq!(dto, expected_dto);
    }
}
