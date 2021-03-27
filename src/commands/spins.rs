use std::iter::FromIterator;

use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::{Duration, sleep};

use crate::apis::*;
use crate::util::util::get_rand_char;

#[command]
async fn dan(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let posts = dan_api::get_posts(None).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.image(str);
                    e.color(10898598);

                    e })
            }).await?,
            None => msg.reply(&ctx.http, "No results found.").await?,
        };

    } else {
        let tags = Vec::from_iter(args.iter::<String>()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>());
        let posts = dan_api::get_posts(Some(tags)).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.image(str);
                    e.color(10898598);

                    e })
            }).await?,
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
            Some(str) => msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.image(str);
                    e.color(10898598);

                    e })
            }).await?,
            None => msg.reply(&ctx.http, "No results found.").await?,
        };

    } else {
        let tags = Vec::from_iter(args.iter::<String>()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>());
        let posts = yan_api::get_posts(Some(tags)).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.image(str);
                    e.color(10898598);

                    e })
            }).await?,
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
            Some(str) => msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.image(str);
                    e.color(10898598);

                    e })
            }).await?,
            None => msg.reply(&ctx.http, "No results found.").await?,
        };

    } else {
        let tags = Vec::from_iter(args.iter::<String>()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>());
        let posts = kona_api::get_posts(Some(tags)).await.unwrap();
        let post_url = posts.get_random_post();
        match post_url {
            Some(str) => msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.image(str);
                    e.color(10898598);

                    e })
            }).await?,
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
            Some(str) => msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.image(str);
                    e.color(10898598);

                    e })
            }).await?,
            None => msg.reply(&ctx.http, "No results found.").await?,
        };
    } else {
        let tags = Vec::from_iter(args.iter::<String>()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>());
        let url = safe_api::get_random_post(Some(tags)).await;
        match url{
            Some(str) => msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.image(str);
                    e.color(10898598);

                    e })
            }).await?,
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
            for i in (0..num+1).rev() {
                let tag = dan_api::get_tags(Some(get_rand_char())).await?.get_random_tag();
                let tag_url = format!("https://danbooru.donmai.us/posts?tags={}", tag);
                let tag_vec = vec![tag.clone()];
                let image = dan_api::get_posts(Some(tag_vec)).await?.get_random_post().unwrap();
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(format!("Tag: [{}]({}) \t Remaining: {}", tag, tag_url, i));
                        e.color(58853)
                    })
                }).await?;

                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.image(image);
                        e.color(10898598)
                    })
                }).await?;

                sleep(Duration::from_secs(2)).await;
            }
        }
    };

    Ok(())
}
