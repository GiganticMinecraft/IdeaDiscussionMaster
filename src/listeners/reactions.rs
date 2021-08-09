use serenity::{
    async_trait,
    model::channel::Reaction,
    prelude::{Context, EventHandler},
};

pub struct ReactionAddedEventHandler;

#[async_trait]
impl EventHandler for ReactionAddedEventHandler {
    async fn reaction_add(&self, _: Context, reaction: Reaction) {
        println!("{}", reaction.emoji);
    }
}
