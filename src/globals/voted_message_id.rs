use serenity::prelude::TypeMapKey;
use std::sync::{atomic::AtomicU64, Arc};

pub struct VotedMessageId;

impl TypeMapKey for VotedMessageId {
    type Value = Arc<AtomicU64>;
}
