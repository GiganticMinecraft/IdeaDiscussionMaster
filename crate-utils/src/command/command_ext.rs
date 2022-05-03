use super::CommandInteraction;

use anyhow::{anyhow, Context};
use serenity::{
    async_trait, builder::CreateEmbed, http::Http, model::interactions::InteractionResponseType,
};

#[async_trait]
pub trait CommandExt {
    async fn message<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        message: T,
    ) -> anyhow::Result<()>;
    async fn embed(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embed: CreateEmbed,
    ) -> anyhow::Result<()>;
}

#[async_trait]
impl CommandExt for CommandInteraction {
    async fn message<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        message: T,
    ) -> anyhow::Result<()> {
        self.create_interaction_response(http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| m.content(message.to_string()))
        })
        .await
        .with_context(|| anyhow!("Cannot to create interaction response!"))
    }

    async fn embed(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embed: CreateEmbed,
    ) -> anyhow::Result<()> {
        self.create_interaction_response(http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| m.add_embed(embed))
        })
        .await
        .with_context(|| anyhow!("Cannot to create interaction response!"))
    }
}
