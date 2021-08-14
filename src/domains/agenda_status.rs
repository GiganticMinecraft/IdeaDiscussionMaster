use itertools::Itertools;
use strum::{Display, EnumIter, EnumProperty, EnumString, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumProperty, EnumString, PartialEq)]
pub enum AgendaStatus {
    #[strum(ascii_case_insensitive, props(ja = "新規", emoji = "🆕", id = "1"))]
    New,
    #[strum(
        ascii_case_insensitive,
        props(ja = "承認", emoji = "⭕", is_done = "true", id="17")
    )]
    Approved,
    #[strum(
        ascii_case_insensitive,
        props(ja = "却下", emoji = "❌", is_done = "true", id = "6")
    )]
    Declined,
}

#[allow(clippy::op_ref)]
impl AgendaStatus {
    pub fn emoji(self) -> String {
        self.get_str("emoji").unwrap().to_string()
    }

    pub fn id(self) -> u16 {
        self.get_str("id").and_then(|str| str.parse::<u16>().ok()).unwrap_or(1)
    }

    pub fn ja(self) -> String {
        self.get_str("ja").unwrap().to_string()
    }

    pub fn from(ch: &char) -> Option<Self> {
        Self::iter().find(|status| status.emoji() == ch.to_string())
    }

    pub fn from_ja(str: &str) -> Option<Self> {
        Self::iter().find(|status| str == &status.ja())
    }

    pub fn from_alias(str: &str) -> Option<Self> {
        Self::iter().find(|status| status.to_string().to_lowercase().starts_with(str))
    }

    pub fn done_statuses() -> Vec<Self> {
        Self::iter()
            .filter(|status| status.get_str("is_done").is_some())
            .collect_vec()
    }
}
