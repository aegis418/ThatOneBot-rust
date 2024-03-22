use std::env;
use std::path::Path;

use poise::{CreateReply, serenity_prelude as serenity};
use rusqlite::{Connection, params, Result};
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::builder::CreateEmbed;
use crate::{Context, Error};

use crate::util::util;

struct TagsDbConnection {
    conn: rusqlite::Connection
}

struct Tag {
    name: String,
    content: String,
}

impl TagsDbConnection {
    fn new() -> TagsDbConnection {
        let dir = env::var("BOT_STORAGE_LOCATION").expect("Cannot find BOT_STORAGE_LOCATION in env.");
        let path = Path::new(dir.as_str()).join("tags.db");
        TagsDbConnection {
            conn: Connection::open(path).expect("Could not open database file.")
        }
    }

    fn close(self) {
        match self.conn.close() {
            Ok(_) => (),
            Err(_) => println!("Failed to close database connection!"),
        };
    }

    fn insert(&mut self, tag: String, content: String) -> Result<usize> {
        let mut stmt = self.conn.prepare("INSERT INTO tags (Name, Content) VALUES (?, ?)")?;
        stmt.execute(params![tag, content])
    }

    fn remove(&mut self, tag: String) -> Result<usize> {
        let mut stmt = self.conn.prepare("DELETE FROM tags WHERE Name=?")?;
        stmt.execute(params![tag])
    }

    fn update(&mut self, tag: String, content: String) -> Result<usize> {
        let mut stmt = self.conn.prepare("UPDATE tags SET Content=? WHERE Name=?")?;
        stmt.execute(params![content, tag])
    }

    fn find_tag(&mut self, tag: String) -> Option<Tag> {
        let mut stmt = self.conn.prepare("SELECT * from tags").ok()?;
        let tag_iter = stmt.query_map(params![], |row| {
            Ok(Tag {
                name: row.get(0)?,
                content: row.get(1)?,
            })
        }).ok()?;

        for t in tag_iter {
            let t = t.unwrap();
            if t.name.eq(&tag) {
               return Some(t)
            }
        }
        None
    }

    fn get_all_tag_names(&mut self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT * FROM tags").unwrap();
        let tag_iter = stmt.query_map(params![], |row| {
            Ok(Tag {
                name: row.get(0)?,
                content: row.get(1)?,
            })
        }).unwrap();

        let name_iter = tag_iter.map(|t| {
            t.unwrap().name
        })
            .collect();

        name_iter
    }
}

#[poise::command(prefix_command,
    aliases("t"),
    category = "Tags",
    subcommands("tag_add", "tag_remove", "tag_update", "tag_raw", "tag_help"))]
pub async fn tag(ctx: Context<'_>, tag: String) -> Result<(), Error> {
    // msg.reply(&ctx.http, "In tag.").await?;
    let mut conn = TagsDbConnection::new();
    let result = conn.find_tag(tag);
    match result {
        Some(t) => {
            if t.content.starts_with("images/") {
                let image_path =
                    Path::new(env::var("BOT_STORAGE_LOCATION").expect("BOT_STORAGE_LOCATION not in environment.").as_str())
                        .join(&t.content);
                // let path_vec = vec![CreateAttachment::path(image_path.to_str().unwrap()).await?];
                // msg.channel_id.send_files(&ctx.http, path_vec, CreateMessage::new()).await?;
                ctx.send(CreateReply::default().attachment(CreateAttachment::path(image_path.to_str().unwrap()).await.unwrap())).await?;
            } else if util::string_ends_with_any(&t.content, vec![".jpg", ".jpeg", ".png", ".gif", ".gifv"]) {
                // msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(CreateEmbed::new().image(&t.content))).await?
                ctx.send(CreateReply::default().embed(CreateEmbed::new().image(&t.content))).await?;
            } else {
                // msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(&t.content)).await?;
                ctx.send(CreateReply::default().content(&t.content)).await?;
            }
        },
        None => { ctx.reply("Tag not found.").await?; },
    };
    conn.close();
    Ok(())
}

#[poise::command(prefix_command, rename = "add", category = "Tags")]
// #[num_args(2)]
async fn tag_add(ctx: Context<'_>, tag_data: Vec<String>) -> Result<(), Error> {
    // msg.reply(&ctx.http, "In tag.add").await?;
    // let name: String = args.single().unwrap();
    // let content: String = args.single().unwrap();
    let mut data_iter = tag_data.into_iter();

    let name = data_iter.next().unwrap();
    let content = data_iter.collect::<Vec<String>>().join(" ");

    let mut conn = TagsDbConnection::new();
    match conn.insert(name, content) {
        Ok(_) => ctx.reply("Successfully added tag.").await?,
        Err(_) => ctx.reply("Failed to add tag.").await?,
    };
    conn.close();

    Ok(())
}

#[poise::command(prefix_command, rename = "remove", aliases("delete", "del", "rm"), category = "Tags")]
// #[num_args(1)]
async fn tag_remove(ctx: Context<'_>, tag_name: String) -> Result<(), Error> {
    // msg.reply(&ctx.http, "In tag.remove").await?;
    // let tag: String = args.single().unwrap();

    let mut conn = TagsDbConnection::new();
    match conn.remove(tag_name) {
        Ok(_) => ctx.reply("Successfully removed tag.").await?,
        Err(_) => ctx.reply("Failed to remove tag.").await?
    };
    conn.close();

    Ok(())
}


#[poise::command(prefix_command, rename = "update", aliases("edit"), category = "Tags")]
// #[num_args(2)]
async fn tag_update(ctx: Context<'_>, tag_data: Vec<String>) -> Result<(), Error> {
    // msg.reply(&ctx.http, "In tag.update").await?;
    // let tag: String = args.single().unwrap();
    // let content: String = args.single().unwrap();

    let mut data_iter = tag_data.into_iter();
    let tag = data_iter.next().unwrap();
    let content = data_iter.collect::<Vec<String>>().join(" ");

    let mut conn = TagsDbConnection::new();
    match conn.update(tag, content) {
        Ok(_) => ctx.reply("Successfully updated tag.").await?,
        Err(_) => ctx.reply("Failed to update tag.").await?,
    };
    conn.close();

    Ok(())
}

#[poise::command(prefix_command, rename = "raw", owners_only, category = "Tags", hide_in_help)]
// #[num_args(1)]
async fn tag_raw(ctx: Context<'_>, tag: String) -> Result<(), Error> {
    // msg.reply(&ctx.http, "In tag.raw").await?;
    // let name: String = args.single().unwrap();

    let mut conn = TagsDbConnection::new();
    let tag = conn.find_tag(tag);
    match tag {
        Some(t) => {
            // msg.channel_id.send_message(&ctx.http, CreateMessage::new().content(format!("```{}```", t.content))).await?
            ctx.send(CreateReply::default().content(format!("```{}```", t.content))).await?;
        },
        None => { ctx.reply("Tag not found.").await?; },
    };
    conn.close();

    Ok(())
}

#[poise::command(prefix_command, rename = "help", category = "Tags")]
async fn tag_help(ctx: Context<'_>) -> Result<(), Error> {
    if let Ok(user_dm_channel) = ctx.author().create_dm_channel(ctx.http()).await {
        let mut conn = TagsDbConnection::new();
        let tag_names = conn.get_all_tag_names();

        let message = CreateMessage::new().content(format!("The list of tags is: ```{}```", tag_names.join("\n")));
        user_dm_channel.send_message(ctx.http(), message).await?;

    } else {
        println!("Failed to create dm channel.");
    }

    Ok(())
}