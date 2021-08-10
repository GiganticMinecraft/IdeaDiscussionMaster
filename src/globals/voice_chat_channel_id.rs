use serenity::prelude::TypeMapKey;
use std::sync::{Arc, atomic::AtomicU64};

pub struct VoiceChatChannelId;

impl TypeMapKey for VoiceChatChannelId {
    type Value = Arc<AtomicU64>;
}
