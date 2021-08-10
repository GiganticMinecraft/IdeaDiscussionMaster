use serenity::{
    async_trait,
    model::{
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}
