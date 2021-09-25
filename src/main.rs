use serenity::{
    framework::{standard::macros::group, StandardFramework},
    prelude::Client,
};
use std::{
    collections::HashMap,
    env,
    sync::Arc,
};
use tokio::sync::RwLock;

use idea_discussion_master::{
    commands::{
        add_agenda::*, end_discussion::*, end_votes::*, help::*, start_discussion::*,
        start_votes::*,
    },
    globals::{
        agendas::Agendas, record_id::RecordId,
        voice_chat_channel_id::VoiceChatChannelId,
    },
    listeners::{after_commands::after, before_commands::before, listener::Handler},
};

#[group]
#[only_in(guilds)]
#[commands(start_discussion, end_discussion, start_votes, end_votes, add_agenda)]
struct General;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DiscordのBot Tokenが見つかりません");

    let framework = StandardFramework::new()
        .configure(|config| config.prefix("\\"))
        .after(after)
        .before(before)
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
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
}
