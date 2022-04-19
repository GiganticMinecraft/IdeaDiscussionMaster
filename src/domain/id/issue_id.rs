#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct IssueId(pub u16);

impl IssueId {
    pub fn new(id: u16) -> Self {
        assert!(id > 0);

        Self(id)
    }
}
