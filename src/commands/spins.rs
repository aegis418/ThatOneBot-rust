use std::io::empty;
use std::iter::FromIterator;
use serenity::all::{CreateEmbed, CreateMessage};
use serenity::all::RoleAction::Create;

use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::{Duration, sleep};

use tracing::{debug};

use crate::apis::*;
use crate::util::util::get_rand_char;

#[command]
async fn dan(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let posts = dan_api::get_posts(None).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => {
                let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
                msg.channel_id.send_message(&ctx.http, message).await?
            },
            None => msg.reply(&ctx.http, "No results found.").await?,
        };

    } else {
        let tags = Vec::from_iter(args.iter::<String>()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>());
        let posts = dan_api::get_posts(Some(tags)).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => {
                let embed = CreateEmbed::new().image(str).color(10898598);
                let message = CreateMessage::new().embed(embed);
                msg.channel_id.send_message(&ctx.http, message).await?
            },
            None => msg.reply(&ctx.http, "No results found.").await?,
        };
    }

    Ok(())
}

#[command]
async fn yan(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let posts = yan_api::get_posts(None).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => {
                let embed = CreateEmbed::new().image(str).color(10898598);
                let message = CreateMessage::new().embed(embed);
                msg.channel_id.send_message(&ctx.http, message).await?
            },
            None => msg.reply(&ctx.http, "No results found.").await?,
        };

    } else {
        let tags = Vec::from_iter(args.iter::<String>()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>());
        let posts = yan_api::get_posts(Some(tags)).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => {
                let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
                msg.channel_id.send_message(&ctx.http, message).await?
            },
            None => msg.reply(&ctx.http, "No results found.").await?,
        };
    }

    Ok(())
}

#[command]
async fn kona(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let posts = kona_api::get_posts(None).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => {
                let embed = CreateEmbed::new().image(str).color(10898598);
                let message = CreateMessage::new().embed(embed);
                msg.channel_id.send_message(&ctx.http, message).await?
            },
            None => msg.reply(&ctx.http, "No results found.").await?,
        };

    } else {
        let tags = Vec::from_iter(args.iter::<String>()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>());
        let posts = kona_api::get_posts(Some(tags)).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => {
                let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
                msg.channel_id.send_message(&ctx.http, message).await?
            },
            None => msg.reply(&ctx.http, "No results found.").await?,
        };
    }

    Ok(())
}

#[command]
async fn safe(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let url = safe_api::get_random_post(None).await;
        match url {
            Some(str) => {
                let embed = CreateEmbed::new().image(str).color(10898598);
                let message = CreateMessage::new().embed(embed);
                msg.channel_id.send_message(&ctx.http, message).await?
            },
            None => msg.reply(&ctx.http, "No results found.").await?,
        };
    } else {
        let tags = Vec::from_iter(args.iter::<String>()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>());
        let url = safe_api::get_random_post(Some(tags)).await;
        match url{
            Some(str) => {
                let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
                msg.channel_id.send_message(&ctx.http, message).await?
            },
            None => msg.reply(&ctx.http, "No results found.").await?,
        };
    }

    Ok(())
}

#[command]
#[aliases("as")]
#[num_args(1)]
async fn auto_spin(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let num = args.single::<u32>().unwrap();
    match num {
        0 => {
            msg.reply(&ctx.http, "Use a number greater than zero.").await?;
        },
        _ => {
            for i in (1..num+1).rev() {
                debug!(i);
                let tag = dan_api::get_tags(Some(get_rand_char())).await?.get_random_tag();
                let tag_url = format!("https://danbooru.donmai.us/posts?tags={}", tag);
                let tag_vec = vec![tag.clone()];
                let image = dan_api::get_posts(Some(tag_vec)).await?.get_random_post().unwrap();
                let tag_embed = CreateEmbed::new().description(format!("Tag: [{}]({}) \t Remaining: {}", tag, tag_url, i)).color(58853);
                let tag_message = CreateMessage::new().embed(tag_embed);
                msg.channel_id.send_message(&ctx.http, tag_message).await?;

                let embed = CreateEmbed::new().image(image).color(10898598);
                let message = CreateMessage::new().embed(embed);
                msg.channel_id.send_message(&ctx.http, message).await?;

                sleep(Duration::from_secs(3)).await;
            }
        }
    };

    Ok(())
}
