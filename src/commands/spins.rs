use poise::{CreateReply, serenity_prelude as serenity};

use serenity::all::CreateEmbed;

use tokio::time::{Duration, sleep};

use tracing::{debug};

use crate::apis::*;
use crate::util::util::get_rand_char;
use crate::{Context, Error};

#[poise::command(prefix_command)]
pub async fn dan(ctx: Context<'_>, tags: Option<String>) -> Result<(), Error> {
    // if args.is_empty() {
    //     let posts = dan_api::get_posts(None).await.unwrap();
    //     let post_url = posts.get_random_post();
    //     match post_url {
    //         Some(str) => {
    //             let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
    //             msg.channel_id.send_message(&ctx.http, message).await?
    //         },
    //         None => msg.reply(&ctx.http, "No results found.").await?,
    //     };
    //
    // } else {
    //     let tags = Vec::from_iter(args.iter::<String>()
    //         .map(|x| x.unwrap())
    //         .collect::<Vec<String>>());
    //     let posts = dan_api::get_posts(Some(tags)).await.unwrap();
    //     let post_url = posts.get_random_post();
    //     match post_url {
    //         Some(str) => {
    //             let embed = CreateEmbed::new().image(str).color(10898598);
    //             let message = CreateMessage::new().embed(embed);
    //             msg.channel_id.send_message(&ctx.http, message).await?
    //         },
    //         None => msg.reply(&ctx.http, "No results found.").await?,
    //     };
    // }

    match tags {
        None => {
            let posts = dan_api::get_posts(None).await.unwrap();
            let post_url = posts.get_random_post();
            match post_url {
                Some(str) => {
                    // let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
                    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(str).color(10898598))).await?;
                },
                None => { ctx.reply("No results found.").await?; },
            };
        }
        Some(list) => {
            // let tags = Vec::from_iter(args.iter::<String>()
            //     .map(|x| x.unwrap())
            //     .collect::<Vec<String>>());
            let lst: Vec<&str> = list.split(" ").collect();
            let posts = dan_api::get_posts(Some(lst)).await.unwrap();
            let post_url = posts.get_random_post();
            match post_url {
                Some(str) => {
                    // let embed = CreateEmbed::new().image(str).color(10898598);
                    // let message = CreateMessage::new().embed(embed);
                    // msg.channel_id.send_message(&ctx.http, message).await?;
                    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(str).color(10898598))).await?;
                },
                None => { ctx.reply("No results found.").await?; },
            };
        }
    }

    Ok(())
}

#[poise::command(prefix_command, category = "Spins")]
pub async fn yan(ctx: Context<'_>, tags: Option<String>) -> Result<(), Error> {
    // if args.is_empty() {
    //     let posts = yan_api::get_posts(None).await.unwrap();
    //     let post_url = posts.get_random_post();
    //     match post_url {
    //         Some(str) => {
    //             let embed = CreateEmbed::new().image(str).color(10898598);
    //             let message = CreateMessage::new().embed(embed);
    //             msg.channel_id.send_message(&ctx.http, message).await?
    //         },
    //         None => msg.reply(&ctx.http, "No results found.").await?,
    //     };
    //
    // } else {
    //     let tags = Vec::from_iter(args.iter::<String>()
    //         .map(|x| x.unwrap())
    //         .collect::<Vec<String>>());
    //     let posts = yan_api::get_posts(Some(tags)).await.unwrap();
    //     let post_url = posts.get_random_post();
    //     match post_url {
    //         Some(str) => {
    //             let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
    //             msg.channel_id.send_message(&ctx.http, message).await?
    //         },
    //         None => msg.reply(&ctx.http, "No results found.").await?,
    //     };
    // }
    match tags {
        None => {
            let posts = yan_api::get_posts(None).await.unwrap();
            let post_url = posts.get_random_post();
            match post_url {
                Some(str) => {
                    // let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
                    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(str).color(10898598))).await?;
                },
                None => { ctx.reply("No results found.").await?; },
            };
        }
        Some(list) => {
            // let tags = Vec::from_iter(args.iter::<String>()
            //     .map(|x| x.unwrap())
            //     .collect::<Vec<String>>());
            let lst = list.split(" ").collect::<Vec<&str>>();
            let posts = yan_api::get_posts(Some(lst)).await.unwrap();
            let post_url = posts.get_random_post();
            match post_url {
                Some(str) => {
                    // let embed = CreateEmbed::new().image(str).color(10898598);
                    // let message = CreateMessage::new().embed(embed);
                    // msg.channel_id.send_message(&ctx.http, message).await?;
                    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(str).color(10898598))).await?;
                },
                None => { ctx.reply("No results found.").await?; },
            };
        }
    }

    Ok(())
}

#[poise::command(prefix_command, category = "Spins")]
pub async fn kona(ctx: Context<'_>, tags: Option<String>) -> Result<(), Error> {
    // if args.is_empty() {
    //     let posts = kona_api::get_posts(None).await.unwrap();
    //     let post_url = posts.get_random_post();
    //     match post_url {
    //         Some(str) => {
    //             let embed = CreateEmbed::new().image(str).color(10898598);
    //             let message = CreateMessage::new().embed(embed);
    //             msg.channel_id.send_message(&ctx.http, message).await?
    //         },
    //         None => msg.reply(&ctx.http, "No results found.").await?,
    //     };
    //
    // } else {
    //     let tags = Vec::from_iter(args.iter::<String>()
    //         .map(|x| x.unwrap())
    //         .collect::<Vec<String>>());
    //     let posts = kona_api::get_posts(Some(tags)).await.unwrap();
    //     let post_url = posts.get_random_post();
    //     match post_url {
    //         Some(str) => {
    //             let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
    //             msg.channel_id.send_message(&ctx.http, message).await?
    //         },
    //         None => msg.reply(&ctx.http, "No results found.").await?,
    //     };
    // }

    match tags {
        None => {
            let posts = kona_api::get_posts(None).await.unwrap();
            let post_url = posts.get_random_post();
            match post_url {
                Some(str) => {
                    // let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
                    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(str).color(10898598))).await?;
                },
                None => { ctx.reply("No results found.").await?; },
            };
        }
        Some(list) => {
            // let tags = Vec::from_iter(args.iter::<String>()
            //     .map(|x| x.unwrap())
            //     .collect::<Vec<String>>());
            let lst = list.split(" ").collect::<Vec<&str>>();
            let posts = kona_api::get_posts(Some(lst)).await.unwrap();
            let post_url = posts.get_random_post();
            match post_url {
                Some(str) => {
                    // let embed = CreateEmbed::new().image(str).color(10898598);
                    // let message = CreateMessage::new().embed(embed);
                    // msg.channel_id.send_message(&ctx.http, message).await?;
                    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(str).color(10898598))).await?;
                },
                None => { ctx.reply("No results found.").await?; },
            };
        }
    }
    Ok(())
}

#[poise::command(prefix_command, category = "Spins")]
pub async fn safe(ctx: Context<'_>, tags: Option<String>) -> Result<(), Error> {
    // if args.is_empty() {
    //     let url = safe_api::get_random_post(None).await;
    //     match url {
    //         Some(str) => {
    //             let embed = CreateEmbed::new().image(str).color(10898598);
    //             let message = CreateMessage::new().embed(embed);
    //             msg.channel_id.send_message(&ctx.http, message).await?
    //         },
    //         None => msg.reply(&ctx.http, "No results found.").await?,
    //     };
    // } else {
    //     let tags = Vec::from_iter(args.iter::<String>()
    //         .map(|x| x.unwrap())
    //         .collect::<Vec<String>>());
    //     let url = safe_api::get_random_post(Some(tags)).await;
    //     match url{
    //         Some(str) => {
    //             let message = CreateMessage::new().embed(CreateEmbed::new().image(str).color(10898598));
    //             msg.channel_id.send_message(&ctx.http, message).await?
    //         },
    //         None => msg.reply(&ctx.http, "No results found.").await?,
    //     };
    // }

    match tags {
        None => {
            let url = safe_api::get_random_post(None).await;
            match url {
                None => { ctx.reply("No results found.").await?; }
                Some(url) => {
                    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(url).color(10898598))).await?;
                }
            }
        }
        Some(list) => {
            let lst = list.split(" ").collect::<Vec<&str>>();
            let url = safe_api::get_random_post(Some(lst)).await;
            match url {
                None => {
                    ctx.reply("No results found.").await?;
                }
                Some(url) => {
                    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(url).color(10898598))).await?;
                }
            }
        }
    }

    Ok(())
}

#[poise::command(prefix_command, aliases("as"), category = "Spins")]
pub async fn auto_spin(ctx: Context<'_>, num_spins: u32) -> Result<(), Error> {
    match num_spins {
        0 => {
            ctx.reply("Use a number greater than zero.").await?;
        },
        _ => {
            for i in (1..num_spins+1).rev() {
                debug!(i);
                let tag = dan_api::get_tags(Some(get_rand_char())).await?.get_random_tag();
                let tag_url = format!("https://danbooru.donmai.us/posts?tags={}", tag);
                let tag_vec = vec![tag.as_ref()];
                let image = dan_api::get_posts(Some(tag_vec)).await?.get_random_post().unwrap();
                let tag_embed = CreateEmbed::new().description(format!("Tag: [{}]({}) \t Remaining: {}", tag, tag_url, i)).color(58853);
                // let tag_message = CreateMessage::new().embed(tag_embed);
                // msg.channel_id.send_message(&ctx.http, tag_message).await?;
                ctx.send(CreateReply::default().embed(tag_embed)).await?;

                let embed = CreateEmbed::new().image(image).color(10898598);
                // let message = CreateMessage::new().embed(embed);
                // msg.channel_id.send_message(&ctx.http, message).await?;
                ctx.send(CreateReply::default().embed(embed)).await?;

                sleep(Duration::from_secs(3)).await;
            }
        }
    };

    Ok(())
}
