use c_domain::status::{AgendaStatus, RecordStatus};

use strum::EnumProperty;

pub trait UseStatusEmoji {
    fn emoji(&self) -> String
    where
        Self: EnumProperty,
    {
        self.get_str("emoji").unwrap().to_string()
    }
}

impl UseStatusEmoji for AgendaStatus {}
