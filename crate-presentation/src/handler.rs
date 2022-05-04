use super::command;
use crate_utils::{
    command::{
        application_interaction::{ApplicationInteractions, SlashCommand},
        ArgsMap, CommandExt, CommandInteraction, InteractionResponse, Parser,
    },
    SerenityContext,
};

use anyhow::{anyhow, Context};
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::EventHandler,
    http::client::Http,
    model::{
        gateway::Ready,
        interactions::{application_command::ApplicationCommand, Interaction},
    },
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: SerenityContext, _data_about_bot: Ready) {
        if let Err(e) = create_slash_commands(&ctx.http)
            .await
            .context("Error while creating slash commands")
        {
            println!("{:?}", e);
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

    async fn interaction_create(&self, ctx: SerenityContext, interaction: Interaction) {
        // TODO: is guild? check
        if let Some(command) = interaction.clone().application_command() {
            let response = create_interaction(&command, &ctx)
                .await
                .unwrap_or_else(|m| InteractionResponse::Message(format!("{:?}", m)));

            let _ = match response {
                InteractionResponse::Message(m) => command.message(&ctx.http, m).await,
                InteractionResponse::Embed(e) => command.embed(&ctx.http, e).await,
            };
        }
    }
}

async fn create_slash_commands(http: impl AsRef<Http>) -> anyhow::Result<()> {
    let guild_id = serenity::model::id::GuildId(std::env::var("GUILD_ID")?.parse()?); // TODO: env
    let commands = command::all_commands()?;
    let _ = serenity::model::id::GuildId::set_application_commands(&guild_id, &http, |command| {
        command.set_application_commands(commands)
    })
    .await?;

    Ok(())
}

async fn create_interaction(
    interaction: &CommandInteraction,
    context: &SerenityContext,
) -> anyhow::Result<InteractionResponse> {
    let (cmd, args) = split_of(interaction)?;
    let sub_command = args.get("sub_command").and_then(|i| match i {
        ApplicationInteractions::SlashCommand(SlashCommand::SubCommand(name)) => Some(name.clone()),
        _ => None,
    });
    let fn_args = (args, context.to_owned(), interaction.to_owned());

    let error = anyhow!("予期していないコマンドです。");
    let result = match cmd.as_str() {
        "start" => command::start::executor(fn_args).await,
        "end" => command::end::executor(fn_args).await,
        "vote" => command::vote::executor(fn_args).await,
        "agenda" => match sub_command.unwrap().as_str() {
            "add" => command::agenda::add(fn_args).await,
            "list" => command::agenda::list(fn_args).await,
            "set" => command::agenda::set(fn_args).await,
            _ => Err(error),
        },
        "create" => match sub_command.unwrap().as_str() {
            "new_record" => command::create::new_record(fn_args).await,
            "issue" => command::create::issue(fn_args).await,
            _ => Err(error),
        },
        _ => Err(error),
    }
    .context("コマンドの処理中にエラーが発生しました。")?;

    Ok(match result {
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

fn split_of(interaction: &CommandInteraction) -> anyhow::Result<(String, ArgsMap)> {
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

    Ok((cmd.to_owned(), args))
}
