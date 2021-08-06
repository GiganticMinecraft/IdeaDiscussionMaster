use serenity::prelude::*;
use std::sync::{atomic::AtomicU16, Arc};

pub struct RecordsId;

impl TypeMapKey for RecordsId {
    type Value = Arc<AtomicU16>;
}