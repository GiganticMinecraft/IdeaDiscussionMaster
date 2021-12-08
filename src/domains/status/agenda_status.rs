use itertools::Itertools;
use strum::{Display, EnumIter, EnumProperty, EnumString, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumProperty, EnumString, PartialEq)]
pub enum AgendaStatus {
    #[strum(ascii_case_insensitive, props(ja = "新規", emoji = "🆕", id = "1"))]
    New,
    #[strum(ascii_case_insensitive, props(ja = "進行中", emoji = "▶", id = "2"))]
    InProgress,
    #[strum(
        ascii_case_insensitive,
        props(ja = "承認", emoji = "⭕", is_done = "true", id = "17")
    )]
    Approved,
    #[strum(
        ascii_case_insensitive,
        props(ja = "却下", emoji = "❌", is_done = "true", id = "6")
    )]
    Declined,
}

impl AgendaStatus {
    pub fn emoji(self) -> String {
        self.get_str("emoji").unwrap().to_string()
    }

    pub fn ja(self) -> String {
        self.get_str("ja").unwrap().to_string()
    }

    pub fn from(ch: &char) -> Option<Self> {
        Self::iter().find(|status| status.emoji() == ch.to_string())
    }

    pub fn from_ja(str: &str) -> Option<Self> {
        Self::iter().find(|status| *str == status.ja())
    }

    pub fn from_alias(str: &str) -> Option<Self> {
        Self::iter().find(|status| {
            status
                .to_string()
                .to_lowercase()
                .starts_with(&str.to_lowercase())
        })
    }

    pub fn done_statuses() -> Vec<Self> {
        Self::iter()
            .filter(|status| status.get_str("is_done").is_some())
            .collect_vec()
    }

    pub fn is_done(&self) -> bool {
        Self::done_statuses().contains(self)
    }

    pub fn is_new(&self) -> bool {
        *self == Self::New
    }

    pub fn is_in_progress(&self) -> bool {
        *self == Self::InProgress
    }

    pub fn id(&self) -> u16 {
        self.get_str("id")
            .and_then(|str| str.parse::<u16>().ok())
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod test {
    use super::AgendaStatus;
    use itertools::Itertools;
    use std::str::FromStr;
    use strum::IntoEnumIterator;
    use test_case::test_case;

    #[test_case("new" => AgendaStatus::New; "newから(insensitive)")]
    #[test_case("New" => AgendaStatus::New; "Newから")]
    #[test_case("inprogress" => AgendaStatus::InProgress; "inprogressから(insensitive)")]
    #[test_case("InProgress" => AgendaStatus::InProgress; "inprogressから")]
    #[test_case("approved" => AgendaStatus::Approved; "Approvedから(insensitive)")]
    #[test_case("Approved" => AgendaStatus::Approved; "Approvedから")]
    #[test_case("declined" => AgendaStatus::Declined; "Declinedから(insensitive)")]
    #[test_case("Declined" => AgendaStatus::Declined; "Declinedから")]
    fn agenda_status_from_str(str: &str) -> AgendaStatus {
        AgendaStatus::from_str(str).unwrap()
    }

    #[test_case(AgendaStatus::New => "🆕")]
    #[test_case(AgendaStatus::InProgress => "▶")]
    #[test_case(AgendaStatus::Approved => "⭕")]
    #[test_case(AgendaStatus::Declined => "❌")]
    fn agenda_status_to_emoji(status: AgendaStatus) -> String {
        status.emoji()
    }

    #[test_case(AgendaStatus::New => "新規")]
    #[test_case(AgendaStatus::InProgress => "進行中")]
    #[test_case(AgendaStatus::Approved => "承認")]
    #[test_case(AgendaStatus::Declined => "却下")]
    fn agenda_status_to_ja(status: AgendaStatus) -> String {
        status.ja()
    }

    #[test_case("new" => Some(AgendaStatus::New))]
    #[test_case("inp" => Some(AgendaStatus::InProgress))]
    #[test_case("app" => Some(AgendaStatus::Approved))]
    #[test_case("dec" => Some(AgendaStatus::Declined))]
    fn agenda_status_from_alias_str(str: &str) -> Option<AgendaStatus> {
        AgendaStatus::from_alias(str)
    }

    #[test_case("新規" => Some(AgendaStatus::New))]
    #[test_case("進行中" => Some(AgendaStatus::InProgress))]
    #[test_case("承認" => Some(AgendaStatus::Approved))]
    #[test_case("却下" => Some(AgendaStatus::Declined))]
    fn agenda_status_from_ja(str: &str) -> Option<AgendaStatus> {
        AgendaStatus::from_ja(str)
    }

    #[test_case(AgendaStatus::New => 1)]
    #[test_case(AgendaStatus::InProgress => 2)]
    #[test_case(AgendaStatus::Approved => 17)]
    #[test_case(AgendaStatus::Declined => 6)]
    fn agenda_status_id(status: AgendaStatus) -> u16 {
        status.id()
    }

    #[test]
    fn agenda_statuses_done_statuses() {
        assert_eq!(
            AgendaStatus::done_statuses(),
            vec!(AgendaStatus::Approved, AgendaStatus::Declined)
        );
    }

    #[test]
    fn test_agenda_status_contents() {
        assert_eq!(AgendaStatus::iter().count(), 4);
        assert_eq!(
            AgendaStatus::iter().collect_vec(),
            vec!(
                AgendaStatus::New,
                AgendaStatus::InProgress,
                AgendaStatus::Approved,
                AgendaStatus::Declined
            )
        );
    }
}
