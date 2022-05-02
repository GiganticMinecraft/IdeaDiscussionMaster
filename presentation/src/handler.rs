use crate::command;
use anyhow::Context;
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::EventHandler,
    http::client::Http,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{ApplicationCommand, ApplicationCommandInteraction},
            Interaction,
        },
    },
};
use std::collections::HashMap;
use utils::command::{
    application_interaction::{ApplicationInteractions, SlashCommand},
    CommandExt, InteractionResponse, Parser,
};

#[derive(Debug)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: serenity::client::Context, _data_about_bot: Ready) {
        if let Err(e) = create_slash_commands(&ctx.http)
            .await
            .context("Error while creating slash commands")
        {
            println!("{:#?}", e);
        };

        let interactions = ApplicationCommand::get_global_application_commands(&ctx.http).await;
        if let Ok(commands) = interactions {
            for cmd in commands.iter().filter(|cmd| {
                !command::all_command_names()
                    .iter()
                    .any(|s| cmd.name.starts_with(s))
            }) {
                let _ =
                    ApplicationCommand::delete_global_application_command(&ctx.http, cmd.id).await;
            }
        }

        println!("Botが正常に起動しました");
        // let commands = ApplicationCommand::get_global_application_commands(&ctx.http).await;
        // println!(
        //     "現在登録されているGuildCommandは以下の通りです: {:#?}",
        //     commands
        // );
    }

    async fn interaction_create(&self, ctx: serenity::client::Context, interaction: Interaction) {
        if let Some(command) = interaction.clone().application_command() {
            let response = create_interaction(&command)
                .await
                .unwrap_or_else(|m| InteractionResponse::Message(m.to_string()));

            let _ = match response {
                InteractionResponse::Message(m) => command.message(&ctx.http, m).await,
                InteractionResponse::Embed(e) => command.embed(&ctx.http, e).await,
            };
        }
    }
}

async fn create_slash_commands(http: impl AsRef<Http>) -> anyhow::Result<()> {
    let guild_id = serenity::model::id::GuildId(std::env::var("GUILD_ID")?.parse()?); // TODO: env
    let _ = serenity::model::id::GuildId::set_application_commands(&guild_id, &http, |command| {
        command.set_application_commands(command::all_commands())
    })
    .await?;

    Ok(())
}

async fn create_interaction(
    interaction: &ApplicationCommandInteraction,
) -> anyhow::Result<InteractionResponse> {
    let data = interaction.data.parse()?;
    let (cmd, args) = data.split_first().unwrap();
    let cmd = match &cmd.1 {
        ApplicationInteractions::SlashCommand(SlashCommand::Command(cmd))
            if command::all_command_names().contains(cmd) =>
        {
            Ok(cmd)
        }
        _ => Err(anyhow::anyhow!("Unexpected interaction")),
    }?;
    let args = args.iter().cloned().collect::<HashMap<_, _>>();

    let response = command::executor(cmd)(args)
        .await
        .context("Error while creating a response")?;

    Ok(match response {
        InteractionResponse::Message(m) if m == *"" => {
            InteractionResponse::Message("Success: There is no message".to_string())
        }
        InteractionResponse::Embed(e) if e.0 == HashMap::default() => {
            let embed = CreateEmbed::default()
                .title("Success")
                .description("There is no message")
                .to_owned();
            InteractionResponse::Embed(embed)
        }
        res => res,
    })
}
