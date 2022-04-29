use super::StatusExt;
use strum::{EnumIter, EnumProperty};

#[derive(Debug, PartialEq, Clone, EnumProperty, EnumIter)]
pub enum RecordStatus {
    #[strum(props(id = "1"))]
    New,
    #[strum(props(id = "5"))]
    Closed,
}

impl Default for RecordStatus {
    fn default() -> Self {
        Self::New
    }
}

impl StatusExt for RecordStatus {
    fn is_new(&self) -> bool {
        *self == Self::New
    }

    fn is_closed(&self) -> bool {
        *self == Self::Closed
    }
}
