use derive_new::new;

#[derive(new, Debug, PartialEq, Eq, Hash, Clone, Default)]
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

#[derive(new, PartialEq, Eq, Hash, Clone)]
pub struct MessageId(u64);

#[derive(new)]
pub struct ChannelId(u64);

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(1; "1")]
    #[test_case(39; "39")]
    #[test_case(100; "100")]
    fn agenda_id_into(num: u16) {
        let id = AgendaId::new(num);

        assert_eq!(num, id.into());
    }

    #[test_case(1; "1")]
    #[test_case(39; "39")]
    #[test_case(100; "100")]
    fn record_id_into(num: u16) {
        let id = RecordId::new(num);

        assert_eq!(num, id.into());
    }
}
