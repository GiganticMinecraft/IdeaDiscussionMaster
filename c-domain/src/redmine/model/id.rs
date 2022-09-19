use derive_new::new;

#[derive(new, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct AgendaId(pub u16);

#[derive(new, Debug, PartialEq, Eq, Clone, Default)]
pub struct RecordId(pub u16);
