use serde::Deserialize;
use strum::{Display, EnumIter, EnumProperty, IntoEnumIterator};

#[derive(
    EnumProperty,
    EnumIter,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Deserialize,
)]
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

    pub fn is_approved(&self) -> bool {
        *self == Self::Approved
    }

    pub fn is_closed(&self) -> bool {
        *self == Self::Declined || self.is_approved()
    }

    pub fn closed() -> Vec<Self> {
        Self::iter().filter(|status| status.is_closed()).collect()
    }

    pub fn from_string(str: &str) -> Option<Self> {
        Self::iter().find(|status| str == status.to_string())
    }
}

impl Default for AgendaStatus {
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
    fn status_is_new(status: AgendaStatus) -> bool {
        status.is_new()
    }

    #[test]
    fn default_status() {
        assert_eq!(AgendaStatus::default(), AgendaStatus::New);
    }

    #[test_case("id"; "id")]
    #[test_case("ja"; "ja")]
    #[test_case("emoji"; "emoji")]
    fn all_props_is_set(props: &str) {
        assert!(AgendaStatus::iter()
            .map(|s| s.get_str(props))
            .all(|opt| opt.is_some()))
    }
}
