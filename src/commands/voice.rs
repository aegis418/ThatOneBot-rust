use std::sync::{Arc, RwLock};

use poise::{CreateReply, serenity_prelude as serenity};
use regex::Regex;
use serenity::prelude::TypeMapKey;
use songbird::input::{AuxMetadata, Compose, HttpRequest, YoutubeDl};
use songbird::tracks::TrackHandle;

use crate::{Context, Error};
use crate::apis::ocremix_api::*;
use crate::HttpKey;

// TODO: Expand on this using a hashmap to allow multiple guilds.
#[derive(Clone, Debug)]
pub enum NowPlaying {
    None,
    Youtube {
        track: TrackHandle,
        meta: Arc<AuxMetadata>
    },
    OCRemix {
        track: TrackHandle,
        playing: OCRemix
    }
}

impl TypeMapKey for NowPlaying {
    type Value = Arc<RwLock<NowPlaying>>;
}

#[poise::command(prefix_command, guild_only, category = "Voice")]
pub async fn join(ctx: Context<'_>)  -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild.voice_states
            .get(ctx.author().id.as_ref())
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.reply("Not in a voice channel").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Did not init songbird in client builder.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[poise::command(prefix_command, guild_only, category = "Voice")]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Songbird not initialized").clone();

    if manager.get(guild_id).is_some() {
        if let Err(e) = manager.remove(guild_id).await {
            ctx.say(format!("Failed: {:?}", e)).await?;
        }
    } else {
        ctx.reply("Not in a voice channel").await?;
    }

    Ok(())
}

#[poise::command(prefix_command, guild_only, subcommands("play_ocremix"), category = "Voice")]
pub async fn play(ctx: Context<'_>, source: String) -> Result<(), Error> {
    // let url = match args.single::<String>() {
    //     Ok(url) => url,
    //     Err(_) => {
    //         msg.channel_id.say(&ctx.http, "Must provide a URL or ID to a video or audio").await?;
    //
    //         return Ok(());
    //
    //     },
    // };
    if source.is_empty() {
        ctx.reply("Must provide URL or ID to a video or audio source.").await?;
        return Ok(());
    }


    let re = Regex::new(r"(?m)^([a-zA-Z0-9_\-]{11,})$").unwrap();

    if !source.starts_with("http") && !re.is_match(&*source) {
        ctx.reply("Must provide a valid URL").await?;

        return Ok(());
    }

    let guild_id = ctx.guild_id().unwrap();

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>().cloned().expect("Should be in typemap.")
    };

    let manager = songbird::get(ctx.serenity_context()).await
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

        let mut src = YoutubeDl::new(http_client, source);

        let track_handle = handler.play_input(src.clone().into());

        // Update global now playing.
        {

            let metadata = src.aux_metadata().await.unwrap();
            let np_handle = ctx.data().now_playing.clone();
            let mut now_playing = np_handle.write().unwrap();

            *now_playing = NowPlaying::Youtube { track: track_handle.clone(), meta: Arc::from(metadata) }

        }


        ctx.reply("Playing song").await?;
    } else {
        ctx.reply("Not in a voice channel to play in").await?;
    }

    Ok(())
}

#[poise::command(prefix_command, guild_only, rename = "ocremix", category = "Voice")]
pub async fn play_ocremix(ctx: Context<'_>, station_source: Option<String>) -> Result<(), Error> {
    // let station: String = if !args.is_empty() {
    //     args.single::<String>().unwrap()
    // } else {
    //     String::from("")
    // };
    // let station_id = StationID::from(station);
    // let stream_url = station_id.get_stream_url().await;

    let station = match station_source {
        None => { String::from("") }
        Some(s) => { s }
    };
    let station_id = StationID::from(station);
    let stream_url = station_id.get_stream_url().await;


    let guild_id = ctx.guild_id().unwrap();

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>().cloned().expect("Should be in typemap.")
    };

    let manager = songbird::get(ctx.serenity_context()).await
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

        // Update global now playing.
        {
            let cur_song = get_current_song(station_id).await.unwrap();
            let np_handle = ctx.data().now_playing.clone();
            let mut now_playing = np_handle.write().unwrap();

            *now_playing = NowPlaying::OCRemix {
                track: track_handle.clone(),
                playing: cur_song
            }

        }

    }

    Ok(())
}

#[poise::command(prefix_command, category = "Voice")]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Songbird not initialized").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Update global now playing.
        {
            let np_handle = ctx.data().now_playing.clone();
            let mut now_playing = np_handle.write().unwrap();

            *now_playing = NowPlaying::None;

        }

        handler.stop();

    } else {
        ctx.reply("Not in a voice channel").await?;
    }

    Ok(())
}

async fn update_now_playing(ctx: &Context<'_>) {
    // let now_playing_lock = {
    //     let data_read = ctx.data.read().await;
    //     data_read.get::<NowPlaying>().expect("Expected NowPlaying in TypeMap.").clone()
    // };
    //
    // let cur_info_lock = now_playing_lock.read().await;
    // let cur_info = cur_info_lock.deref().clone();

    let cur_info = {
        ctx.data().now_playing.clone().read().unwrap().clone()
    };
    // println!("{:?}", cur_info);
    match cur_info {
        NowPlaying::None => {return;}
        NowPlaying::Youtube { .. } => {return}
        NowPlaying::OCRemix { playing, track } => {
            {
                let cur_song = get_current_song(playing.station_id).await.unwrap();
                let np_handle = ctx.data().now_playing.clone();
                let mut np = np_handle.write().unwrap();
                *np = NowPlaying::OCRemix {
                    track: track.clone(),
                    playing: cur_song
                };
            }
        }
    }


}

#[poise::command(prefix_command, guild_only, aliases("np"), category = "Voice")]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), Error> {
    update_now_playing(&ctx).await;

    let now_playing_info = {
        ctx.data().now_playing.clone().read().unwrap().clone()
    };
    {
        match now_playing_info {
            NowPlaying::None => {
                ctx.reply("Nothing is playing").await?;
            }
            NowPlaying::Youtube { track: _, meta } => {
                // let metadata = track.metadata();
                let embed = serenity::CreateEmbed::new().title(String::from(meta.title.as_ref().unwrap())).url(meta.source_url.as_ref().unwrap()).color(16741516);
                ctx.send(CreateReply::default().embed(embed)).await?;
            }
            NowPlaying::OCRemix { playing, track: _ } => {

                let url = match playing.url.as_ref() {
                    None => {String::from("")}
                    Some(url) => {String::from(url)}
                };

                let station_name: &String = &playing.station_id.into();
                let embed = serenity::CreateEmbed::new().color(10276252)
                    .title(&playing.title)
                    .url(url)
                    .thumbnail(&playing.album_url)
                    .description(format!("Album: {}\nStation: {}", playing.album, station_name));
                ctx.send(CreateReply::default().embed(embed)).await?;
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

// async fn get_metadata(ctx: Context<'_>) -> AuxMetadata {
//     let manager = songbird::get(ctx.serenity_context()).await
//         .expect("Songbird Voice client placed in at initialisation.").clone();
//
//     manager.
//
//     if let Some(handler_lock) = manager.get(ctx.guild_id().unwrap()) {
//         let mut handler = handler_lock.lock().await;
//
//         handler.
//     }
//     todo!()
// }