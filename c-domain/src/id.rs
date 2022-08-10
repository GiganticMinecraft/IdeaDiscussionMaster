use derive_new::new;

#[derive(new, Debug, PartialEq, Clone, Default)]
pub struct AgendaId(u16);

impl From<AgendaId> for u16 {
    fn from(id: AgendaId) -> Self {
        id.0
    }
}

#[derive(new, Debug, PartialEq, Clone, Default)]
pub struct RecordId(u16);

impl From<RecordId> for u16 {
    fn from(id: RecordId) -> Self {
        id.0
    }
}

#[derive(new)]
pub struct MessageId(u64);

#[derive(new)]
pub struct ChannelId(u64);
