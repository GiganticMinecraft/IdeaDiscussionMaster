// TODO: rename this file's name to record_status

#[derive(Debug, PartialEq, Clone)]
pub enum RecordStatus {
    New,
    Closed,
}

impl Default for RecordStatus {
    fn default() -> Self {
        Self::New
    }
}
