use serenity::{
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::Client,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::{
        atomic::{AtomicU16, AtomicU64},
        Arc,
    },
};
use tokio::sync::RwLock;

use idea_discussion_master::{
    commands::{end_discussion::*, start_discussion::*, start_votes::*},
    globals::{
        agendas::Agendas, current_agenda_id::CurrentAgendaId, record_id::RecordId,
        voice_chat_channel_id::VoiceChatChannelId, voted_message_id::VotedMessageId,
    },
    listeners::{after_commands::after, before_commands::before, listener::Handler},
};

#[group]
#[commands(start_discussion, end_discussion, start_votes)]
struct General;

// TODO: expectをなくす

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("IDEA_DISCUSSION_MASTER_DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let owners = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            owners
        }
        Err(reason) => panic!("Could not access application info: {:?}", reason),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("\\"))
        .after(after)
        .before(before)
        .group(&GENERAL_GROUP);

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
