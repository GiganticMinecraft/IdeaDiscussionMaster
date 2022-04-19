#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RecordId(pub u16);

impl RecordId {
    pub fn new(id: u16) -> Self {
        Self(id)
    }
}
