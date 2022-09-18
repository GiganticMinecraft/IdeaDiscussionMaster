use c_domain::redmine::model::id::{AgendaId, RecordId};

pub trait UseFormattedId {
    fn formatted(&self) -> String;
}

impl UseFormattedId for AgendaId {
    fn formatted(&self) -> String {
        format!("#{}", self.0)
    }
}

impl UseFormattedId for RecordId {
    fn formatted(&self) -> String {
        format!("#{}", self.0)
    }
}
