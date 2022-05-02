use derive_new::new;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, new)]
pub struct IssueId(pub u16);
