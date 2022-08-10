use strum::{EnumIter, EnumProperty};

#[derive(EnumProperty, EnumIter, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AgendaStatus {
    #[strum(props(ja = "新規", emoji = "🆕", id = "1"))]
    New,
    #[strum(props(ja = "進行中", emoji = "▶", id = "2"))]
    InProgress,
    #[strum(props(ja = "承認", emoji = "⭕", id = "17"))]
    Approved,
    #[strum(props(ja = "却下", emoji = "❌", id = "6"))]
    Declined,
}

impl AgendaStatus {
    pub fn is_new(&self) -> bool {
        *self == Self::New
    }
}

impl Default for AgendaStatus {
    fn default() -> Self {
        Self::New
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumProperty, EnumIter)]
pub enum RecordStatus {
    #[strum(props(id = "1"))]
    New,
    #[strum(props(id = "5"))]
    Closed,
}

impl RecordStatus {
    pub fn is_new(&self) -> bool {
        *self == Self::New
    }

    pub fn is_closed(&self) -> bool {
        *self == Self::Closed
    }
}

impl Default for RecordStatus {
    fn default() -> Self {
        Self::New
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use strum::IntoEnumIterator;
    use test_case::test_case;

    #[test_case(AgendaStatus::New => true; "new")]
    #[test_case(AgendaStatus::InProgress => false; "in_progress")]
    #[test_case(AgendaStatus::Declined => false; "declined")]
    #[test_case(AgendaStatus::Approved => false; "approved")]
    fn agenda_status_is_new(status: AgendaStatus) -> bool {
        status.is_new()
    }

    #[test]
    fn default_agenda_status() {
        assert_eq!(AgendaStatus::default(), AgendaStatus::New);
    }

    #[test_case("id"; "id")]
    #[test_case("ja"; "ja")]
    #[test_case("emoji"; "emoji")]
    fn all_agenda_status_props_is_set(props: &str) {
        assert!(AgendaStatus::iter()
            .map(|s| s.get_str(props))
            .all(|opt| opt.is_some()))
    }

    #[test_case(RecordStatus::New => true; "new")]
    #[test_case(RecordStatus::Closed => false; "closed")]
    fn record_status_is_new(status: RecordStatus) -> bool {
        status.is_new()
    }

    #[test_case(RecordStatus::New => false; "new")]
    #[test_case(RecordStatus::Closed => true; "closed")]
    fn record_status_is_closed(status: RecordStatus) -> bool {
        status.is_closed()
    }

    #[test]
    fn default_record_status() {
        assert_eq!(RecordStatus::default(), RecordStatus::New);
    }

    #[test_case("id"; "id")]
    fn all_record_status_props_is_set(props: &str) {
        assert!(RecordStatus::iter()
            .map(|s| s.get_str(props))
            .all(|opt| opt.is_some()))
    }
}
