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

        assert_eq!(dto.id, agenda.id.into());
        assert_eq!(dto.title, agenda.title);
        assert_eq!(dto.description, agenda.description);
        assert_eq!(dto.status, agenda.status);
    }
}
