use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;

use songbird::SerenityInit;

use serenity::{
    async_trait,
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::*,
    },
    http::Http,
    model::{gateway::Ready, channel::Message},
    prelude::*
};

use commands::{
    spins::*,
    tags::*,
    utility::*,
    voice::*,
};

use regex::*;

use tracing::{info, error};

mod apis;
mod util;
mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        let re = Regex::new(r"^.*(ifunny.co/picture).*$").unwrap();
        if re.is_match(message.content.as_str()) {
            let url = ifunny_replace(&message);
            message.channel_id.send_message(&ctx.http, |m| m.content(url)).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is ready.", ready.user.name);
    }
}

#[group]
#[commands(get_avatar, boxes)]
struct General;

#[group]
#[commands(dan, yan, kona, safe, auto_spin)]
struct Spins;

#[group]
#[commands(tag)]
struct Tags;

#[group]
#[commands(join, leave, play, stop, now_playing)]
struct Voice;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load from .env file.");
    tracing_subscriber::fmt().init();

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
        .group(&SPINS_GROUP)
        .group(&TAGS_GROUP)
        .group(&VOICE_GROUP);

    // Build the bot client
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error creating client.");

    // Register NowPlaying into client global data.
    {
        let mut data = client.data.write().await;

        data.insert::<NowPlaying>(Arc::new(RwLock::new(NowPlaying::None)))
    }

    // Start the client.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
