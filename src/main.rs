use anyhow::Context;
use idea_discussion_master::{
    presentation::{global::module, Handler},
    util,
};
use serenity::client::Client;

// TODO: move main.rs

async fn build_bot_client() -> anyhow::Result<Client> {
    let util::Env {
        discord_token,
        discord_application_id,
        ..
    } = util::Env::new();

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
