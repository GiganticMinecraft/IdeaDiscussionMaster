use serenity::prelude::TypeMapKey;
use std::sync::{atomic::AtomicU16, Arc};

pub struct RecordId;

impl TypeMapKey for RecordId {
    type Value = Arc<AtomicU16>;
}