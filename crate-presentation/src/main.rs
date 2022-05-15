use crate_presentation::{global::module, Handler};

use anyhow::Context;
use serenity::{client::Client, model::gateway::GatewayIntents};

async fn build_bot_client() -> anyhow::Result<Client> {
    let crate_shared::Env {
        discord_token,
        discord_application_id,
        ..
    } = crate_shared::Env::new();

    Client::builder(discord_token, GatewayIntents::GUILDS)
        .application_id(discord_application_id)
        .event_handler(Handler)
        .await
        .context("クライアントの作成に失敗しました")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    module::init().await;

    let mut client = build_bot_client().await.unwrap();
    if let Err(reason) = client.start().await {
        eprintln!("クライアントの起動に失敗しました: {:?}", reason);
    }

    Ok(())
}
