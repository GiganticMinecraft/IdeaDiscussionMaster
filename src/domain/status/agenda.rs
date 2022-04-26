use strum::{Display, EnumIter, EnumProperty, EnumString};

// TODO: rename this file's name to agenda_status

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumProperty, EnumString, PartialEq, Eq, Hash)]
pub enum AgendaStatus {
    #[strum(props(ja = "新規", emoji = "🆕"))]
    New,
    #[strum(props(ja = "進行中", emoji = "▶"))]
    InProgress,
    #[strum(props(ja = "承認", emoji = "⭕"))]
    Approved,
    #[strum(props(ja = "却下", emoji = "❌"))]
    Declined,
}

impl AgendaStatus {
    pub fn is_closed(&self) -> bool {
        *self == Self::Approved || *self == Self::Declined
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
