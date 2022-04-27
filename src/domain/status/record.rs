use super::StatusExt;
use strum::EnumProperty;

// TODO: rename this file's name to record_status

#[derive(Debug, PartialEq, Clone, EnumProperty)]
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

impl StatusExt for RecordStatus {}
