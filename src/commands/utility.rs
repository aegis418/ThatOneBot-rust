use std::env;
use std::path::Path;

use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::{Duration, sleep};

#[command]
#[aliases("ga")]
async fn get_avatar(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let uid = if args.is_empty() {
        msg.author.id
    } else {
        let id: u64 = args.single().unwrap();
        UserId::from(id)
    };

    let user = uid.to_user(&ctx.http).await;
    match user {
        Ok(usr) => msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.image(usr.avatar_url().unwrap());

                e
            });

            m
        }).await?,
        Err(_err) => msg.reply(&ctx.http, "No user with given id").await?
    };

    Ok(())
}

#[command]
#[aliases("box")]
#[description("Displays an image up to 5 times to help clear the screen.")]
async fn boxes(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let image_path =
        Path::new(env::var("BOT_STORAGE_LOCATION").expect("BOT_STORAGE_LOCATION not in environment.").as_str())
        .join("images")
        .join("box.png");

    let num = if args.is_empty() {
        4
    } else {
        let arg: i32 = args.single().unwrap();
        if arg > 5 {
            5
        } else if arg < 1 {
            1
        } else {
            arg
        }
    };

    for _ in 0..num {
        let list = vec![image_path.to_str().unwrap()];
        msg.channel_id.send_files(&ctx.http, list, |m| m ).await?;
        sleep(Duration::from_millis(500)).await;
    };

    Ok(())
}