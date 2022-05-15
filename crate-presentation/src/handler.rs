use crate::command;
use crate_shared::{
    command::{
        application_interaction::{ApplicationInteractions, SlashCommand},
        CommandExt, CommandInteraction,
    },
    SerenityContext,
};

use anyhow::{anyhow, ensure, Context};
use serenity::{
    async_trait,
    client::EventHandler,
    http::client::Http,
    model::{
        gateway::Ready,
        interactions::{application_command::ApplicationCommand, Interaction},
    },
};

#[derive(Debug)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: SerenityContext, _data_about_bot: Ready) {
        if let Err(e) = create_slash_commands(&ctx.http).await {
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
        if let Some(command) = interaction.clone().application_command() {
            let _ = command.defer(&ctx.http).await;

            if let Err(e) = create_interaction(&command, &ctx).await {
                let _ = command.message(&ctx.http, format!("{:?}", e)).await;
            }
        }
    }
}

async fn create_slash_commands(http: impl AsRef<Http>) -> anyhow::Result<()> {
    let guild_id = serenity::model::id::GuildId(std::env::var("GUILD_ID")?.parse()?); // TODO: env
    let commands =
        command::all_commands().context("SlashCommandの生成中にエラーが発生しました。")?;
    let _ = serenity::model::id::GuildId::set_application_commands(&guild_id, &http, |command| {
        command.set_application_commands(commands)
    })
    .await
    .context("SlashCommandをDiscordに登録している間にエラーが発生しました。")?;

    Ok(())
}

async fn create_interaction(
    interaction: &CommandInteraction,
    ctx: &SerenityContext,
) -> anyhow::Result<()> {
    ensure!(
        interaction.guild_id.is_some(),
        "このコマンドはサーバー内からのみ実行できます。"
    );
    ensure!(
        !interaction.user.bot,
        "このコマンドはユーザーのみが実行できます。"
    );

    let (command, args) = interaction.split_of().await?;
    let sub_command = args
        .get("sub_command")
        .and_then(|i| match i {
            ApplicationInteractions::SlashCommand(SlashCommand::SubCommand(name))
                if command::all_command_names().contains(&command) =>
            {
                Some(name.clone())
            }
            _ => None,
        })
        .unwrap_or_default();
    let fn_args = (args, ctx.to_owned(), interaction.to_owned());

    let error = anyhow!("予期していないコマンドです。");

    match command.as_str() {
        "start" => command::start::executor(fn_args).await,
        "end" => command::end::executor(fn_args).await,
        "vote" => match sub_command.as_str() {
            "start" => command::vote::start(fn_args).await,
            "end" => command::vote::end(fn_args).await,
            _ => Err(error),
        },
        "agenda" => match sub_command.as_str() {
            "add" => command::agenda::add(fn_args).await,
            "list" => command::agenda::list(fn_args).await,
            _ => Err(error),
        },
        "create" => match sub_command.as_str() {
            "new_record" => command::create::new_record(fn_args).await,
            "issue" => command::create::issue(fn_args).await,
            _ => Err(error),
        },
        _ => Err(error),
    }
    .context("ApplicationInteractionの送信中にエラーが発生しました。")
}
