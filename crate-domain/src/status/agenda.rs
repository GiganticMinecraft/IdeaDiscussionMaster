use super::StatusExt;

use serenity::model::channel::ReactionType;
use strum::{Display, EnumIter, EnumProperty, EnumString};

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumIter,
    EnumProperty,
    EnumString,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
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
    pub fn closed() -> Vec<Self> {
        vec![Self::Approved, Self::Declined]
    }

    pub fn is_in_progress(&self) -> bool {
        *self == Self::InProgress
    }

    pub fn ja(&self) -> String {
        self.get_str("ja").unwrap().to_string()
    }

    pub fn emoji(&self) -> String {
        self.get_str("emoji").unwrap().to_string()
    }
}

impl Default for AgendaStatus {
    fn default() -> Self {
        Self::New
    }
}

impl StatusExt for AgendaStatus {
    fn is_new(&self) -> bool {
        *self == Self::New
    }

    fn is_closed(&self) -> bool {
        Self::closed().iter().any(|s| s == self)
    }
}

impl From<AgendaStatus> for ReactionType {
    fn from(status: AgendaStatus) -> Self {
        Self::from(status.emoji().chars().next().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn closed_statuses() {
        assert_eq!(
            AgendaStatus::closed(),
            vec![AgendaStatus::Approved, AgendaStatus::Declined]
        );
    }

    #[test_case(AgendaStatus::New => false; "new is false")]
    #[test_case(AgendaStatus::InProgress => true; "in progress is true")]
    #[test_case(AgendaStatus::Approved => false; "approved is false")]
    #[test_case(AgendaStatus::Declined => false; "declined is false")]
    fn whether_is_in_progress(status: AgendaStatus) -> bool {
        status.is_in_progress()
    }

    #[test_case(AgendaStatus::New => true; "new is true")]
    #[test_case(AgendaStatus::InProgress => false; "in progress is false")]
    #[test_case(AgendaStatus::Approved => false; "approved is false")]
    #[test_case(AgendaStatus::Declined => false; "declined is false")]
    fn whether_is_new(status: AgendaStatus) -> bool {
        status.is_new()
    }

    #[test_case(AgendaStatus::New => false; "new is false")]
    #[test_case(AgendaStatus::InProgress => false; "in progress is false")]
    #[test_case(AgendaStatus::Approved => true; "approved is true")]
    #[test_case(AgendaStatus::Declined => true; "declined is true")]
    fn whether_is_closed(status: AgendaStatus) -> bool {
        status.is_closed()
    }

    #[test_case(AgendaStatus::New => "新規"; "new")]
    #[test_case(AgendaStatus::InProgress => "進行中"; "in progress")]
    #[test_case(AgendaStatus::Approved => "承認"; "approved")]
    #[test_case(AgendaStatus::Declined => "却下"; "declined")]
    fn ja(status: AgendaStatus) -> String {
        status.ja()
    }

    #[test_case(AgendaStatus::New => "🆕"; "new")]
    #[test_case(AgendaStatus::InProgress => "▶"; "in progress")]
    #[test_case(AgendaStatus::Approved => "⭕"; "approved")]
    #[test_case(AgendaStatus::Declined => "❌"; "declined")]
    fn emoji(status: AgendaStatus) -> String {
        status.emoji()
    }

    #[test_case(AgendaStatus::New => 1; "new")]
    #[test_case(AgendaStatus::InProgress => 2; "in progress")]
    #[test_case(AgendaStatus::Approved => 17; "approved")]
    #[test_case(AgendaStatus::Declined => 6; "declined")]
    fn id(status: AgendaStatus) -> u16 {
        status.id()
    }

    #[test_case(1 => Some(AgendaStatus::New); "new")]
    #[test_case(2 => Some(AgendaStatus::InProgress); "in progress")]
    #[test_case(17 => Some(AgendaStatus::Approved); "approved")]
    #[test_case(6 => Some(AgendaStatus::Declined); "declined")]
    #[test_case(50 => None; "undefined number should return None")]
    fn from_id(num: u16) -> Option<AgendaStatus> {
        AgendaStatus::from_id(num)
    }

    #[test]
    fn all() {
        assert_eq!(
            AgendaStatus::all(),
            vec![
                AgendaStatus::New,
                AgendaStatus::InProgress,
                AgendaStatus::Approved,
                AgendaStatus::Declined
            ]
        );
    }
}
