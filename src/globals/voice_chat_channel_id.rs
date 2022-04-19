use once_cell::sync::Lazy;
use serenity::model::id::ChannelId;
use std::sync::{Arc, Mutex};

pub type VoiceChatChannelId = ChannelId;
type VCChIdOpt = Option<VoiceChatChannelId>;

static VOICE_CHAT_CH_ID: Lazy<Arc<Mutex<VCChIdOpt>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn get() -> VCChIdOpt {
    *VOICE_CHAT_CH_ID.lock().unwrap()
}

pub fn update(id: VoiceChatChannelId) -> VCChIdOpt {
    *VOICE_CHAT_CH_ID.lock().unwrap() = Some(id);

    get()
}

pub fn clear() -> VCChIdOpt {
    *VOICE_CHAT_CH_ID.lock().unwrap() = None;

    get()
}

pub fn exists() -> bool {
    VOICE_CHAT_CH_ID.lock().unwrap().is_some()
}
