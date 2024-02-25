use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::tracks::TrackHandle;
use std::sync::Arc;
use songbird::input::{YoutubeDl, HttpRequest, Compose, AuxMetadata};

use regex::Regex;

use crate::apis::ocremix_api::*;
use std::ops::Deref;
use serenity::all::CreateEmbed;
use serenity::builder::CreateMessage;
use crate::HttpKey;

// TODO: Expand on this using a hashmap to allow multiple guilds.
#[derive(Clone, Debug)]
pub enum NowPlaying {
    None,
    Youtube {
        track: TrackHandle,
        meta: AuxMetadata
    },
    OCRemix {
        track: TrackHandle,
        playing: OCRemix
    }
}

impl TypeMapKey for NowPlaying {
    type Value = Arc<RwLock<NowPlaying>>;
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild_id, channel_id) = {
        let guild = msg.guild(&ctx.cache).unwrap();
        let channel_id = guild.voice_states
            .get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(&ctx.http, "Not in a voice channel").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Did not init songbird in client builder.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird not initialized").clone();

    if manager.get(guild_id).is_some() {
        if let Err(e) = manager.remove(guild_id).await {
            msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await?;
        }
    } else {
        msg.reply(&ctx.http, "Not in a voice channel").await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[sub_commands(play_ocremix)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Must provide a URL or ID to a video or audio").await?;

            return Ok(());
        },
    };

    let re = Regex::new(r"(?m)^([a-zA-Z0-9_\-]{11,})$").unwrap();

    if !url.starts_with("http") && !re.is_match(&*url) {
        msg.channel_id.say(&ctx.http, "Must provide a valid URL").await?;

        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();

    let http_client = {
        let data = ctx.data.read().await;
        data.get::<HttpKey>().cloned().expect("Should be in typemap.")
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // let mut source = match songbird::ytdl(&url).await {
        //     Ok(source) => source,
        //     Err(why) => {
        //         println!("Err starting source: {:?}", why);
        //
        //         msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await?;
        //
        //         return Ok(());
        //     },
        // };

        let mut src = YoutubeDl::new(http_client, url);

        let track_handle = handler.play_input(src.clone().into());

        let now_playing_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<NowPlaying>().expect("Expected NowPlaying in TypeMap.").clone()
        };

        // Update global now playing.
        {
            let mut now_playing = now_playing_lock.write().await;
            let metadata = src.aux_metadata().await.unwrap();

            *now_playing = NowPlaying::Youtube { track: track_handle.clone(), meta: metadata }

        }


        msg.channel_id.say(&ctx.http, "Playing song").await?;
    } else {
        msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await?;
    }

    Ok(())
}

#[command("ocremix")]
#[only_in(guilds)]
async fn play_ocremix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let station: String = if !args.is_empty() {
        args.single::<String>().unwrap()
    } else {
        String::from("")
    };
    let station_id = StationID::from(station);
    let stream_url = station_id.get_stream_url().await;

    let guild_id = msg.guild_id.unwrap();

    let http_client = {
        let data = ctx.data.read().await;
        data.get::<HttpKey>().cloned().expect("Should be in typemap.")
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird not initialized").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // let source = match songbird::ffmpeg(&*stream_url).await {
        //     Ok(source) => source,
        //     Err(why) => {
        //         println!("Err starting source: {:?}", why);
        //
        //         msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await?;
        //
        //         return Ok(());
        //     },
        // };

        let src = HttpRequest::new(http_client, stream_url);

        let track_handle = handler.play_input(src.clone().into());

        let now_playing_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<NowPlaying>().expect("Expected NowPlaying in TypeMap.").clone()
        };

        // Update global now playing.
        {
            let mut now_playing = now_playing_lock.write().await;

            *now_playing = NowPlaying::OCRemix {
                track: track_handle.clone(),
                playing: get_current_song(station_id).await.unwrap()
            }

        }

    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird not initialized").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let now_playing_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<NowPlaying>().expect("Expected NowPlaying in TypeMap.").clone()
        };

        // Update global now playing.
        {
            let mut now_playing = now_playing_lock.write().await;

            *now_playing = NowPlaying::None;

        }

        handler.stop();

    } else {
        msg.reply(&ctx.http, "Not in a voice channel").await?;
    }

    Ok(())
}

async fn update_now_playing(ctx: &Context) {
    let now_playing_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<NowPlaying>().expect("Expected NowPlaying in TypeMap.").clone()
    };

    let cur_info_lock = now_playing_lock.read().await;
    let cur_info = cur_info_lock.deref().clone();
    // println!("{:?}", cur_info);
    drop(cur_info_lock);
    match cur_info {
        NowPlaying::None => {return;}
        NowPlaying::Youtube { .. } => {return}
        NowPlaying::OCRemix { playing, track } => {
            {
                let mut np = now_playing_lock.write().await;
                *np = NowPlaying::OCRemix {
                    track: track.clone(),
                    playing: get_current_song(playing.station_id).await.unwrap()
                };
            }
        }
    }


}

#[command]
#[only_in(guilds)]
#[aliases("np")]
async fn now_playing(ctx: &Context, msg: &Message) -> CommandResult {
    update_now_playing(ctx).await;

    let now_playing_lock = ctx.data.read().await;
    let now_playing = now_playing_lock.get::<NowPlaying>().expect("Expected NowPlaying in data").clone();

    {
        let now_playing_info =  now_playing.read().await;

        match now_playing_info.deref() {
            NowPlaying::None => {
                msg.channel_id.say(&ctx.http, "Nothing is playing").await?;
            }
            NowPlaying::Youtube { track, meta } => {
                // let metadata = track.metadata();
                let embed = CreateEmbed::new().title(String::from(meta.title.as_ref().unwrap())).url(meta.source_url.as_ref().unwrap()).color(16741516);
                msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
            }
            NowPlaying::OCRemix { playing, track: _ } => {

                let url = match playing.url.as_ref() {
                    None => {String::from("")}
                    Some(url) => {String::from(url)}
                };

                let station_name: &String = &playing.station_id.into();
                let embed = CreateEmbed::new().color(10276252)
                    .title(&playing.title)
                    .url(url)
                    .thumbnail(&playing.album_url)
                    .description(format!("Album: {}\nStation: {}", playing.album, station_name));
                msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
                // msg.channel_id.send_message(&ctx.http, |m| {
                //     m.embed(|e| {
                //         e.color(10276252);
                //         e.title(&playing.title);
                //         e.url(url);
                //         e.thumbnail(&playing.album_url);
                //         let station_name: &String = &playing.station_id.into();
                //         e.description(format!("Album: {} \nStation: {}", playing.album, station_name))
                //     })
                // }).await?;
            }
        }

    }


    Ok(())
}