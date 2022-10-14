use c_domain::redmine::model::status::AgendaStatus;

use strum::EnumProperty;

pub trait UseStatusEmoji {
    fn emoji(&self) -> char
    where
        Self: EnumProperty,
    {
        self.get_str("emoji").and_then(|s| s.parse().ok()).unwrap()
    }
}

impl UseStatusEmoji for AgendaStatus {}
