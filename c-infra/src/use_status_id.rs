use c_domain::status::{AgendaStatus, RecordStatus};

use strum::{EnumProperty, IntoEnumIterator};

pub trait UseStatusId {
    fn id(&self) -> u16
    where
        Self: EnumProperty,
    {
        self.get_str("id")
            .and_then(|str| str.parse().ok())
            .unwrap_or(1)
    }

    fn from_id(id: u16) -> Self
    where
        Self: IntoEnumIterator + EnumProperty + Default,
    {
        Self::iter().find(|s| s.id() == id).unwrap_or_default()
    }
}

impl UseStatusId for AgendaStatus {}

impl UseStatusId for RecordStatus {}
