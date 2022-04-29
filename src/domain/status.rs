pub mod agenda;
pub use agenda::AgendaStatus;

pub mod record;
pub use record::RecordStatus;

use strum::{EnumProperty, IntoEnumIterator};

pub trait StatusExt {
    fn id(&self) -> u16
    where
        Self: EnumProperty,
    {
        self.get_str("id")
            .and_then(|str| str.parse::<u16>().ok())
            .unwrap_or(1)
    }

    fn from_id(id: u16) -> Option<Self>
    where
        Self: IntoEnumIterator + EnumProperty,
    {
        Self::iter().find(|s| s.id() == id)
    }

    fn is_new(&self) -> bool;

    fn is_closed(&self) -> bool;
}
