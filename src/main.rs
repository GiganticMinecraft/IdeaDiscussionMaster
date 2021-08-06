use serenity::{
    framework::{
        standard::macros::{group, hook},
        StandardFramework,
    },
    http::Http,
    model::channel::Message,
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::{
        atomic::{AtomicU16, AtomicUsize},
        Arc,
    },
};
use tokio::sync::RwLock;

use idea_discussion_master::{
    commands::{end_discussion::*, start_discussion::*},
    globals::{records_id::RecordsId, CommandCounter, MessageCount},
    listeners::ready::ReadyEventHandler,
};

#[group]
#[commands(start_discussion, end_discussion)]
struct General;

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!(
        "Running command '{}' invoked by '{}'",
        command_name,
        msg.author.tag()
    );

    let counter_lock = {
        let data_read = ctx.data.read().await;

        data_read
            .get::<CommandCounter>()
            .expect("Expected CommandCounter in TypeMap.")
            .clone()
    };

    {
        let mut counter = counter_lock.write().await;

        let entry = counter.entry(command_name.to_string()).or_insert(0);
        *entry += 1;
    }

    true
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("IDEA_DISCUSSION_MASTER_DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
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

        data.insert::<CommandCounter>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<MessageCount>(Arc::new(AtomicUsize::new(0)));
        data.insert::<RecordsId>(Arc::new(AtomicU16::new(0)));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
