use super::{
    application_interaction::{ApplicationInteractions, SlashCommand},
    ArgsMap, CommandInteraction, Parser,
};

use anyhow::Context;
use itertools::Itertools;
use serenity::{async_trait, builder::CreateEmbed, http::Http, model::id::MessageId};
use std::collections::HashMap;

#[async_trait]
pub trait CommandExt {
    async fn message<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        message: T,
    ) -> anyhow::Result<MessageId>;
    async fn messages<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        messages: Vec<T>,
    ) -> anyhow::Result<MessageId>;
    async fn embed(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embed: CreateEmbed,
    ) -> anyhow::Result<MessageId>;
    async fn embeds(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embeds: Vec<CreateEmbed>,
    ) -> anyhow::Result<MessageId>;

    async fn split_of(&self) -> anyhow::Result<(String, ArgsMap)>;
}

#[async_trait]
impl CommandExt for CommandInteraction {
    async fn message<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        message: T,
    ) -> anyhow::Result<MessageId> {
        self.messages(http, vec![message]).await
    }

    async fn messages<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        messages: Vec<T>,
    ) -> anyhow::Result<MessageId> {
        self.edit_original_interaction_response(http, |resp| {
            resp.content(messages.iter().map(|msg| msg.to_string()).join("\n"))
        })
        .await
        .map(|msg| msg.id)
        .context("Cannot to edit interaction response!")
    }

    async fn embed(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embed: CreateEmbed,
    ) -> anyhow::Result<MessageId> {
        self.embeds(http, vec![embed]).await
    }

    async fn embeds(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embeds: Vec<CreateEmbed>,
    ) -> anyhow::Result<MessageId> {
        self.edit_original_interaction_response(http, |resp| resp.set_embeds(embeds))
            .await
            .map(|msg| msg.id)
            .context("Cannot to edit interaction response!")
    }

    async fn split_of(&self) -> anyhow::Result<(String, ArgsMap)> {
        let data = self.data.parse()?;
        let (cmd, args) = data.split_first().unwrap();
        let cmd = match &cmd.1 {
            ApplicationInteractions::SlashCommand(SlashCommand::Command(cmd)) => Ok(cmd),
            _ => Err(anyhow::anyhow!("Unexpected interaction")),
        }?;
        let args = args.iter().cloned().collect::<HashMap<_, _>>();

        Ok((cmd.to_owned(), args))
    }
}
