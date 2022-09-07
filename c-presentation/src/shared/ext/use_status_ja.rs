use c_domain::status::{AgendaStatus, RecordStatus};

use strum::EnumProperty;

pub trait UseStatusJa {
    fn ja(&self) -> String
    where
        Self: EnumProperty,
    {
        self.get_str("ja").unwrap().to_string()
    }
}

impl UseStatusJa for AgendaStatus {}

impl UseStatusJa for RecordStatus {}
