#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AgendaId(pub u16);

impl AgendaId {
    pub fn new(id: u16) -> Self {
        assert!(id > 0);

        Self(id)
    }
}
