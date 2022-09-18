use c_domain::redmine::model::status::AgendaStatus;

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
