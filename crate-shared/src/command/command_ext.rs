use super::CommandInteraction;

use anyhow::{anyhow, Context};
use itertools::Itertools;
use serenity::{async_trait, builder::CreateEmbed, http::Http};

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
        let _ = self
            .edit_original_interaction_response(http, |resp| {
                resp.content(messages.iter().map(|msg| msg.to_string()).join("\n"))
            })
            .await
            .context(anyhow!("Cannot to edit interaction response!"))?;

        Ok(())
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
        let _ = self
            .edit_original_interaction_response(http, |resp| resp.set_embeds(embeds))
            .await
            .with_context(|| anyhow!("Cannot to edit interaction response!"))?;

        Ok(())
    }
}
