use serenity::{
    framework::{
        standard::macros::{group, hook},
        StandardFramework,
    },
    http::Http,
    model::channel::Message,
    prelude::{Context, Client},
};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::{atomic::AtomicU16, Arc},
};
use tokio::sync::RwLock;

use idea_discussion_master::{
    commands::{end_discussion::*, start_discussion::*},
    globals::{agendas::Agendas, record_id::RecordId},
    listeners::ready::ReadyEventHandler,
};

#[group]
#[commands(start_discussion, end_discussion)]
struct General;

#[hook]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    println!(
        "Running command '{}' invoked by '{}'",
        command_name,
        msg.author.tag()
    );

    true
}

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
        .before(before)
        .group(&GENERAL_GROUP);

    // TODO: helpとか該当コマンドなしとか？
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(ReadyEventHandler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<RecordId>(Arc::new(AtomicU16::new(0)));
        data.insert::<Agendas>(Arc::new(RwLock::new(HashMap::default())));
    }

    if let Err(reason) = client.start().await {
        println!("Client error: {:?}", reason);
    }
}
