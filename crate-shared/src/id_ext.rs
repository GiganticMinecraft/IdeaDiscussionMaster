use crate_domain::id::IssueId;

pub trait IdExt {
    fn formatted(&self) -> String;
}

// TODO: これに統一
impl IdExt for IssueId {
    fn formatted(&self) -> String {
        format!("#{}", self.0)
    }
}
