use std::env;
use std::sync::{Arc, RwLock};

use regex::*;

use reqwest::Client as HttpClient;

use poise::serenity_prelude as serenity;
use serenity::all::FullEvent;

use tracing::{error};

use commands::{
    spins::*,
    tags::*,
    utility::*,
    voice::*,
};

mod apis;
mod util;
mod commands;


type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {
    http: HttpClient,
    songbird: Arc<songbird::Songbird>,
    now_playing: Arc<RwLock<NowPlaying>>
}


#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load from .env file.");
    tracing_subscriber::fmt().init();

    // Get token from env var.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token env variable");

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::privileged();

    let manager = songbird::Songbird::serenity();
    let manager_clone = manager.clone();

    let framework = poise::Framework::builder()
        .setup(|_context, _ready, _framework| {
            Box::pin(async move {
                Ok(Data{
                    http: HttpClient::new(),
                    songbird: manager_clone,
                    now_playing: Arc::new(RwLock::new(NowPlaying::None))
                })
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![
                get_avatar(),
                boxes(),
                dan(),
                yan(),
                kona(),
                safe(),
                auto_spin(),
                tag(),
                join(),
                leave(),
                play(),
                stop(),
                now_playing(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(";".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            initialize_owners: true,
            ..Default::default()
        }).build();


    // Build the bot client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .voice_manager_arc(manager)
        .await
        .expect("Error creating client.");

    // Start the client.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as : {}", data_about_bot.user.name);
        }

        FullEvent::Message { new_message } => {
            let re = Regex::new(r"^.*(ifunny.co/picture).*$").unwrap();
            if re.is_match(new_message.content.as_str()) {
                let url = ifunny_replace(new_message);
                let mess = serenity::CreateMessage::new().content(url);
                let _ = new_message.channel_id.send_message(&ctx.http, mess).await;
            }
        }

        FullEvent::VoiceStateUpdate { old, new, .. } => {
            match old {
                None => {}
                Some(state) => {
                    match state.channel_id {
                        None => {}
                        Some(cid) => {
                            // Disconnect bot if it is the only connection left in a voice channel.
                            let channel = cid.to_channel(&ctx.http).await.unwrap().guild().unwrap();
                            let members = channel.members(&ctx.cache).unwrap();
                            // println!("OLD: {:?}", members);
                            if members.len() == 1 {
                                if members[0].user.bot {
                                    let _ = songbird::get(&ctx).await.unwrap().remove(channel.guild_id).await;
                                }
                            }
                        }
                    }
                }
            }
            // Do something when someone connects.
            //new
        }

        _ => {}
    }

    Ok(())
}
