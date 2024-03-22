use std::env;
use std::path::Path;

use poise::{CreateReply, serenity_prelude as serenity};
use tokio::time::{Duration, sleep};

use serenity::Message;

use scraper::{Html, Selector};
use serenity::builder::{CreateAttachment, CreateEmbed};
use crate::{Context, Error};

#[poise::command(prefix_command, aliases("ga"), category = "Utility")]
pub async fn get_avatar(ctx: Context<'_>, userid: Option<u64>) -> Result<(), Error> {
    // let uid = if args.is_empty() {
    //     ctx.author().id
    // } else {
    //     let id: u64 = args.single().unwrap();
    //     UserId::from(id)
    // };

    let uid = match userid {
        None => { ctx.author().id }
        Some(id) => { serenity::UserId::from(id) }
    };

    let user = uid.to_user(ctx.http()).await;
    match user {
        Ok(usr) => {
            // let message = CreateMessage::new().embed(CreateEmbed::new().image(usr.avatar_url().unwrap()));
            // ctx.send(message).await?;
            ctx.send(CreateReply::default().embed(CreateEmbed::new().image(usr.avatar_url().unwrap()))).await?;
        },
        Err(_err) => {
            ctx.reply("No user with given id").await?;
        }
    };

    Ok(())
}

#[poise::command(prefix_command, aliases("box"), category = "Utility")]
// #[description("Displays an image up to 5 times to help clear the screen.")]
pub async fn boxes(ctx: Context<'_>, number: Option<u32>) -> Result<(), Error> {
    let image_path =
        Path::new(env::var("BOT_STORAGE_LOCATION").expect("BOT_STORAGE_LOCATION not in environment.").as_str())
        .join("images")
        .join("box.jpg");

    // let num = if args.is_empty() {
    //     4
    // } else {
    //     let arg: i32 = args.single().unwrap();
    //     if arg > 5 {
    //         5
    //     } else if arg < 1 {
    //         1
    //     } else {
    //         arg
    //     }
    // };

    let num = match number {
        None => { 4 }
        Some(n) => {
            if n > 5 {
                5
            } else if n < 1 {
                1
            } else { n }
        }
    };

    for _ in 0..num {
        // let list = vec![CreateAttachment::path(image_path.to_str().unwrap()).await?];
        // msg.channel_id.send_files(&ctx.http, list, CreateMessage::new()).await?;
        ctx.send(CreateReply::default().attachment(CreateAttachment::path(image_path.to_str().unwrap()).await.unwrap())).await?;
        sleep(Duration::from_millis(500)).await;
    };

    Ok(())
}

pub fn ifunny_replace(message: &Message) -> String {
    let url = message.content.as_str();
    let resp = ureq::get(url).call().unwrap();
    let text = resp.into_string().unwrap();

    let doc = Html::parse_document(&text);
    let img_sel = Selector::parse("img").unwrap();

    let new_url = doc
        .select(&img_sel)
        .nth(1)
        .unwrap()
        .value()
        .attr("src")
        .unwrap();

    String::from(new_url)
}