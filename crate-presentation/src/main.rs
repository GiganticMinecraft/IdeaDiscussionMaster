use anyhow::Context;
use presentation::{global::module, Handler};
use serenity::client::Client;

async fn build_bot_client() -> anyhow::Result<Client> {
    let utils::Env {
        discord_token,
        discord_application_id,
        ..
    } = utils::Env::new();

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
