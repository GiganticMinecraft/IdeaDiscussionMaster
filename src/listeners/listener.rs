use serenity::{
    async_trait,
    framework::standard::{Args, Delimiter},
    model::{
        channel::{Reaction, ReactionType},
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
};
use std::str::FromStr;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use crate::domains::redmine_client::MockRedmineClient as RedmineClient;
    } else {
        pub use crate::domains::redmine_client::RedmineClient;
    }
}

use crate::{
    commands::end_votes,
    domains::{discussion, status::agenda_status},
    globals::{agendas, voice_chat_channel_id},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}
