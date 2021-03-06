use crate::{
    command,
    shared::{
        command::{
            application_interaction::{ApplicationInteractions, SlashCommand},
            CommandInteraction,
        },
        ext::CommandExt,
    },
};
use crate_shared::Env;

use anyhow::{anyhow, ensure, Context as _};
use log::{error, info};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::client::Http,
    model::{gateway::Ready, interactions::Interaction},
};

#[derive(Debug)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        if let Err(e) = create_slash_commands(&ctx.http).await {
            error!("{:?}", e);
        };

        info!("Botが正常に起動しました。");

        let commands = ctx
            .http
            .get_guild_application_commands(Env::new().discord_guild_id)
            .await;
        info!(
            "現在登録されているSlashCommandは以下の通りです。: {:#?}",
            commands
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(command) = interaction.clone().application_command() {
            let _ = command.defer(&ctx.http).await;

            if let Err(e) = create_interaction(&command, &ctx).await {
                error!("Create interactions Error: {:?}", e);
                let _ = command.message(&ctx.http, format!("{:?}", e)).await;
            }
        }
    }
}

async fn create_slash_commands(http: impl AsRef<Http>) -> anyhow::Result<()> {
    let guild_id = serenity::model::id::GuildId(Env::new().discord_guild_id);
    let commands =
        command::all_commands().context("SlashCommandの生成中にエラーが発生しました。")?;
    let _ = serenity::model::id::GuildId::set_application_commands(&guild_id, &http, |command| {
        command.set_application_commands(commands)
    })
    .await
    .context("SlashCommandをDiscordに登録している間にエラーが発生しました。")?;

    Ok(())
}

async fn create_interaction(interaction: &CommandInteraction, ctx: &Context) -> anyhow::Result<()> {
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
            "issue" => command::create::issue(fn_args).await,
            "thread" => command::create::thread(fn_args).await,
            "new_record" => command::create::new_record(fn_args).await,
            _ => Err(error),
        },
        _ => Err(error),
    }
}
