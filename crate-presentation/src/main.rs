use crate_presentation::{global::module, Handler};

use anyhow::Context;
use argh::FromArgs;
use log::error;
use serenity::{client::Client, model::gateway::GatewayIntents};

async fn build_bot_client() -> anyhow::Result<Client> {
    let crate_shared::Env {
        discord_token,
        discord_application_id,
        ..
    } = crate_shared::Env::new();

    // NOTICE: Intents
    Client::builder(discord_token, GatewayIntents::non_privileged())
        .application_id(discord_application_id)
        .event_handler(Handler)
        .await
        .context("クライアントの作成に失敗しました。")
}

fn setup_logger(is_verbosed: bool) -> Result<(), fern::InitError> {
    let mut config = fern::Dispatch::new();

    config = if is_verbosed {
        config.level(log::LevelFilter::Debug)
    } else {
        config.level(log::LevelFilter::Info)
    }
    .level_for("serenity", log::LevelFilter::Off)
    .level_for("tracing::span", log::LevelFilter::Off);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout());

    config.chain(stdout_config).apply()?;

    Ok(())
}

#[derive(FromArgs)]
/// Args to execute bot
struct Args {
    /// whether or not to output log verbosely
    #[argh(switch, short = 'v')]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    module::init().await;

    let args: Args = argh::from_env();
    setup_logger(args.verbose).expect("ログの初期化に失敗しました。");

    let mut client = build_bot_client().await.unwrap();
    if let Err(reason) = client.start().await {
        error!("クライアントの起動に失敗しました。: {:?}", reason);
    }

    Ok(())
}
