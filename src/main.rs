use anyhow::{anyhow, Context};
use idea_discussion_master::{
    commands::{self, InteractionResponse},
    globals::{agendas::Agendas, record_id::RecordId, voice_chat_channel_id::VoiceChatChannelId},
    utils::{
        commands::{
            application_interactions::{ApplicationInteractions, SlashCommandType},
            CommandExt, Parser,
        },
        Env,
    },
};
use serenity::{
    async_trait,
    client::{Client, EventHandler},
    http::client::Http,
    model::{
        gateway::Ready,
        interactions::{application_command::ApplicationCommand, Interaction},
    },
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: serenity::client::Context, _data_about_bot: Ready) {
        if let Err(e) = create_slash_commands(&ctx.http).await {
            println!("{:#?}", e);
        };
        let interactions = ApplicationCommand::get_global_application_commands(&ctx.http).await;

        if let Ok(commands) = interactions {
            for cmd in commands.iter().filter(|cmd| {
                !commands::all_command_names()
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
            let data = command.data.parse().unwrap();
            let (cmd, args) = data.split_first().unwrap();
            let cmd = match &cmd.1 {
                ApplicationInteractions::SlashCommand(SlashCommandType::Command(cmd))
                    if commands::all_command_names().contains(cmd) =>
                {
                    Ok(cmd)
                }
                _ => Err(anyhow::anyhow!("Unexpected interaction.")),
            }
            .unwrap();
            let args = args.iter().cloned().collect::<HashMap<_, _>>();

            let result = match commands::executor(cmd)(args) {
                Ok(res) => match res {
                    InteractionResponse::Message(m) => command.message(&ctx.http, m).await,
                    InteractionResponse::Embed(e) => command.embed(&ctx.http, e).await,
                },
                Err(e) => Err(anyhow::anyhow!(
                    "Error has occurred while creating a response: {:#?}",
                    e
                )),
            };
            if let Err(e) = result {
                println!("Error has occurred while executing command: {:#?}", e);
            };
        }
    }
}

async fn build_bot_client() -> anyhow::Result<Client> {
    let Env {
        discord_token,
        discord_application_id,
        ..
    } = Env::new();

    Client::builder(discord_token)
        .application_id(discord_application_id)
        .event_handler(Handler)
        .await
        .with_context(|| anyhow!("Clientの作成に失敗しました"))
}

async fn create_slash_commands(http: impl AsRef<Http>) -> anyhow::Result<()> {
    let guild_id =
        serenity::model::id::GuildId(std::env::var("GUILD_ID").unwrap().parse().unwrap());
    let response =
        serenity::model::id::GuildId::set_application_commands(&guild_id, &http, |command| {
            command.set_application_commands(commands::all_commands())
        })
        .await;

    println!("{:#?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = build_bot_client()
        .await
        .expect("クライアントの作成中にエラーが発生しました");

    {
        let mut data = client.data.write().await;

        data.insert::<RecordId>(Arc::new(RwLock::new(None)));
        data.insert::<Agendas>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<VoiceChatChannelId>(Arc::new(RwLock::new(None)));
    }

    if let Err(reason) = client.start().await {
        eprintln!("クライアントの起動に失敗しました: {:?}", reason);
    }

    Ok(())
}
