use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rusqlite::Connection;
use serenity::{
    async_trait,
    framework::standard::{
        macros::*,
    },
    http::Http,
    model::{gateway::Ready},
    prelude::*
};
use serenity::framework::StandardFramework;

use commands::{
    spins::*,
    tags::*,
    utility::*,
};

mod apis;
mod util;
mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready.", ready.user.name);
    }
}

// struct TagDB;
//
// impl TypeMapKey for TagDB {
//     type Value = Arc<RwLock>;
// }

#[group]
#[commands(get_avatar, boxes)]
struct General;

#[group]
#[commands(dan, yan, kona, safe, auto_spin)]
struct Spins;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load from .env file.");

    // Get token from env var.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token env variable");

    let http = Http::new_with_token(&token);

    let (owner, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owner = HashSet::new();
            owner.insert(info.owner.id);
            (owner, info.id)
        },
        Err(why) => panic!("Could not access app info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .owners(owner)
            .prefix(";"))
        .group(&GENERAL_GROUP)
        .group(&SPINS_GROUP);

    // Build the bot client
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client.");

    // {
    //     let mut db = client.data.write().await;
    //     let db_loc = env::var("BOT_STORAGE_LOCATION")
    //         .expect("Please specify in .env file where you want to store bot related files.");
    //     let db_path = Path::new(&db_loc).join("tags.db");
    //     db.insert::<TagDB>(Arc::new(RwLock::new(Connection::open(db_path).expect("Cannot open/create db file in the location."))))
    // }

    // Start the client.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
