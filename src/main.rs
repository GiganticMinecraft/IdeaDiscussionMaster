use serenity::{
    framework::{standard::macros::group, StandardFramework},
    prelude::Client,
};
use std::{
    collections::HashMap,
    env,
    sync::{
        atomic::{AtomicU16, AtomicU64},
        Arc,
    },
};
use tokio::sync::RwLock;

use idea_discussion_master::{
    commands::{end_discussion::*, end_votes::*, start_discussion::*, start_votes::*, help::*},
    globals::{
        agendas::Agendas, current_agenda_id::CurrentAgendaId, record_id::RecordId,
        voice_chat_channel_id::VoiceChatChannelId, voted_message_id::VotedMessageId,
    },
    listeners::{after_commands::after, before_commands::before, listener::Handler},
};

#[group]
#[only_in(guilds)]
#[commands(start_discussion, end_discussion, start_votes, end_votes)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("IDEA_DISCUSSION_MASTER_DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|config| config.prefix("\\"))
        .after(after)
        .before(before)
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    // TODO: helpとか該当コマンドなしとか？
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<RecordId>(Arc::new(AtomicU16::new(0)));
        data.insert::<Agendas>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<CurrentAgendaId>(Arc::new(AtomicU16::new(0)));
        data.insert::<VotedMessageId>(Arc::new(AtomicU64::new(0)));
        data.insert::<VoiceChatChannelId>(Arc::new(AtomicU64::new(0)));
    }

    if let Err(reason) = client.start().await {
        println!("Client error: {:?}", reason);
    }
}
