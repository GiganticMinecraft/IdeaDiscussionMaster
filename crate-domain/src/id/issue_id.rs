use derive_new::new;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, new, Default)]
pub struct IssueId(pub u16);
