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
