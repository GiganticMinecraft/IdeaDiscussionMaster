use super::command::{
    application_interaction::{ApplicationInteractions, SlashCommand},
    ArgsMap, CommandInteraction, InteractionResponse, Parser,
};

use anyhow::Context;
use itertools::Itertools;
use serenity::{async_trait, builder::CreateEmbed, http::Http, model::channel::Message};
use std::collections::HashMap;

#[async_trait]
pub trait CommandExt {
    async fn message<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        message: T,
    ) -> anyhow::Result<Message>;
    async fn messages<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        messages: Vec<T>,
    ) -> anyhow::Result<Message>;
    async fn embed(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embed: CreateEmbed,
    ) -> anyhow::Result<Message>;
    async fn embeds(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embeds: Vec<CreateEmbed>,
    ) -> anyhow::Result<Message>;
    async fn send(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        response: InteractionResponse,
    ) -> anyhow::Result<Message>;

    async fn split_of(&self) -> anyhow::Result<(String, ArgsMap)>;
}

#[async_trait]
impl CommandExt for CommandInteraction {
    async fn message<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        message: T,
    ) -> anyhow::Result<Message> {
        self.messages(http, vec![message]).await
    }

    async fn messages<T: ToString + Send + Sync>(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        messages: Vec<T>,
    ) -> anyhow::Result<Message> {
        self.edit_original_interaction_response(http, |resp| {
            resp.content(messages.iter().map(|msg| msg.to_string()).join("\n"))
        })
        .await
        .context("Cannot to edit interaction response!")
    }

    async fn embed(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embed: CreateEmbed,
    ) -> anyhow::Result<Message> {
        self.embeds(http, vec![embed]).await
    }

    async fn embeds(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        embeds: Vec<CreateEmbed>,
    ) -> anyhow::Result<Message> {
        self.edit_original_interaction_response(http, |resp| resp.set_embeds(embeds))
            .await
            .context("Cannot to edit interaction response!")
    }

    async fn send(
        &self,
        http: impl AsRef<Http> + Send + Sync + 'async_trait,
        response: InteractionResponse,
    ) -> anyhow::Result<Message> {
        if response.is_empty() {
            return self.send(http, InteractionResponse::default()).await;
        }

        match response {
            InteractionResponse::Message(msg) => self.message(http, msg).await,
            InteractionResponse::Messages(msgs) => self.messages(http, msgs).await,
            InteractionResponse::Embed(embed) => self.embed(http, embed).await,
            InteractionResponse::Embeds(embeds) => self.embeds(http, embeds).await,
        }
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
