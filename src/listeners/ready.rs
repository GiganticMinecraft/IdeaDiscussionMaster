use serenity::{async_trait, model::gateway::Ready, prelude::*};

pub struct ReadyEventHandler;

#[async_trait]
impl EventHandler for ReadyEventHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}