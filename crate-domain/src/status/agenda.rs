use super::StatusExt;

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
