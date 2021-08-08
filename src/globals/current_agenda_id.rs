use serenity::prelude::TypeMapKey;
use std::sync::{atomic::AtomicU16, Arc};

pub struct CurrentAgendaId;

impl TypeMapKey for CurrentAgendaId {
    type Value = Arc<AtomicU16>;
}
