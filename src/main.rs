use anyhow::{anyhow, Context};
use idea_discussion_master::{
    globals::{agendas::Agendas, record_id::RecordId, voice_chat_channel_id::VoiceChatChannelId},
    utils,
};
use serenity::{client::Client};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

async fn build_bot_client() -> anyhow::Result<Client> {
    let utils::Env {
        discord_token,
        discord_application_id,
        ..
    } = utils::Env::new();

    Client::builder(discord_token)
        .application_id(discord_application_id)
        .await
        .with_context(|| anyhow!("Failed to build bot"))
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
