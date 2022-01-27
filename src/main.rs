use idea_discussion_master::{
    commands::{
        add_agenda::*, add_github_issue::*, end_discussion::*, end_votes::*, help::*,
        show_agendas::*, start_discussion::*, start_votes::*,
    },
    globals::{agendas::Agendas, record_id::RecordId, voice_chat_channel_id::VoiceChatChannelId},
    listeners::{self, after_commands, before_commands},
    utils,
};
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    prelude::Client,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[group]
#[only_in(guilds)]
#[commands(
    start_discussion,
    end_discussion,
    start_votes,
    end_votes,
    add_agenda,
    show_agendas,
    add_github_issue
)]
struct General;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|config| config.prefix("\\"))
        .after(after_commands)
        .before(before_commands)
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    let mut client = Client::builder(&utils::Env::new().discord_token)
        .framework(framework)
        .event_handler(listeners::Handler)
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
