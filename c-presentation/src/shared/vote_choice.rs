use crate::shared::ext::UseStatusJa;
use c_domain::redmine::model::status::AgendaStatus;
use derive_new::new;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, new, Hash, PartialEq, Eq, Clone, Debug)]
pub struct VoteChoice {
    pub status: AgendaStatus,
    pub message: String,
}

impl Display for VoteChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status.ja(), self.message)
    }
}

pub type VoteChoiceWithId = (usize, VoteChoice);
