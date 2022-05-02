use crate_presentation::{global::module, Handler};

use anyhow::Context;
use serenity::client::Client;

async fn build_bot_client() -> anyhow::Result<Client> {
    let crate_utils::Env {
        discord_token,
        discord_application_id,
        ..
    } = crate_utils::Env::new();

    Client::builder(discord_token)
        .application_id(discord_application_id)
        .event_handler(Handler)
        .await
        .context("クライアントの作成に失敗しました")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = build_bot_client().await.unwrap();

    module::init().await;

    if let Err(reason) = client.start().await {
        eprintln!("クライアントの起動に失敗しました: {:?}", reason);
    }

    Ok(())
}
