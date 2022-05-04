use super::CommandInteraction;

use anyhow::{anyhow, Context};
use itertools::Itertools;
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
    async fn messages<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        messages: Vec<T>,
    ) -> anyhow::Result<()>;
    async fn embed(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embed: CreateEmbed,
    ) -> anyhow::Result<()>;
    async fn embeds(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embeds: Vec<CreateEmbed>,
    ) -> anyhow::Result<()>;
}

#[async_trait]
impl CommandExt for CommandInteraction {
    async fn message<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        message: T,
    ) -> anyhow::Result<()> {
        self.messages(http, vec![message]).await
    }

    async fn messages<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        messages: Vec<T>,
    ) -> anyhow::Result<()> {
        self.create_interaction_response(http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| {
                    m.content(messages.iter().map(|msg| msg.to_string()).join("\n"))
                })
        })
        .await
        .context(anyhow!("Cannot to create interaction response!"))
    }

    async fn embed(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embed: CreateEmbed,
    ) -> anyhow::Result<()> {
        self.embeds(http, vec![embed]).await
    }

    async fn embeds(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embeds: Vec<CreateEmbed>,
    ) -> anyhow::Result<()> {
        self.create_interaction_response(http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| {
                    embeds.into_iter().for_each(|embed| {
                        m.add_embed(embed);
                    });
                    m
                })
        })
        .await
        .with_context(|| anyhow!("Cannot to create interaction response!"))
    }
}
