use std::env;
use std::path::Path;

use rusqlite::{Connection, params, Result};
use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

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
        stmt.execute(&[tag, content])
    }

    fn remove(&mut self, tag: String) -> Result<usize> {
        let mut stmt = self.conn.prepare("DELETE FROM tags WHERE Name=?")?;
        stmt.execute(&[tag])
    }

    fn update(&mut self, tag: String, content: String) -> Result<usize> {
        let mut stmt = self.conn.prepare("UPDATE tags SET Content=? WHERE Name=?")?;
        stmt.execute(&[content, tag])
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

#[command]
#[aliases("t")]
#[sub_commands(tag_add, tag_remove, tag_update, tag_raw, tag_help)]
#[num_args(1)]
async fn tag(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // msg.reply(&ctx.http, "In tag.").await?;
    let tag: String = args.single().unwrap();
    let mut conn = TagsDbConnection::new();
    let result = conn.find_tag(tag);
    match result {
        Some(t) => {
            if t.content.starts_with("images/") {
                let image_path =
                    Path::new(env::var("BOT_STORAGE_LOCATION").expect("BOT_STORAGE_LOCATION not in environment.").as_str())
                        .join(&t.content);
                let path_vec = vec![image_path.to_str().unwrap()];
                msg.channel_id.send_files(&ctx.http, path_vec, |m| m).await?
            } else if util::string_ends_with_any(&t.content, vec![".jpg", ".jpeg", ".png", ".gif", ".gifv"]) {
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.image(&t.content)
                    })
                }).await?
            } else {
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.content(&t.content)
                }).await?
            }
        },
        None => msg.reply(&ctx.http, "Tag not found.").await?,
    };
    conn.close();
    Ok(())
}

#[command("add")]
#[num_args(2)]
async fn tag_add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // msg.reply(&ctx.http, "In tag.add").await?;
    let name: String = args.single().unwrap();
    let content: String = args.single().unwrap();

    let mut conn = TagsDbConnection::new();
    match conn.insert(name, content) {
        Ok(_) => msg.reply(&ctx.http, "Successfully added tag.").await?,
        Err(_) => msg.reply(&ctx.http, "Failed to add tag.").await?,
    };
    conn.close();

    Ok(())
}

#[command("remove")]
#[aliases("delete", "del", "rm")]
#[num_args(1)]
async fn tag_remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // msg.reply(&ctx.http, "In tag.remove").await?;
    let tag: String = args.single().unwrap();

    let mut conn = TagsDbConnection::new();
    match conn.remove(tag) {
        Ok(_) => msg.reply(&ctx.http, "Successfully removed tag.").await?,
        Err(_) => msg.reply(&ctx.http, "Failed to remove tag.").await?
    };
    conn.close();

    Ok(())
}


#[command("update")]
#[num_args(2)]
async fn tag_update(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // msg.reply(&ctx.http, "In tag.update").await?;
    let tag: String = args.single().unwrap();
    let content: String = args.single().unwrap();

    let mut conn = TagsDbConnection::new();
    match conn.update(tag, content) {
        Ok(_) => msg.reply(&ctx.http, "Successfully updated tag.").await?,
        Err(_) => msg.reply(&ctx.http, "Failed to update tag.").await?,
    };
    conn.close();

    Ok(())
}

#[command("raw")]
#[num_args(1)]
#[owners_only]
async fn tag_raw(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // msg.reply(&ctx.http, "In tag.raw").await?;
    let name: String = args.single().unwrap();

    let mut conn = TagsDbConnection::new();
    let tag = conn.find_tag(name);
    match tag {
        Some(t) => msg.channel_id.send_message(&ctx.http, |m| {
            m.content(format!("```{}```", t.content))
        }).await?,
        None => msg.reply(&ctx.http, "Tag not found.").await?,
    };
    conn.close();

    Ok(())
}

#[command("help")]
async fn tag_help(ctx: &Context, msg: &Message) -> CommandResult {
    if let Ok(user_dm_channel) = msg.author.create_dm_channel(&ctx.http).await {
        let mut conn = TagsDbConnection::new();
        let tag_names = conn.get_all_tag_names();


        user_dm_channel.send_message(&ctx.http, |m| {
            m.content(format!("The list of tags is: ```{}```", tag_names.join("\n")))
        }).await?;

    } else {
        println!("Failed to create dm channel.");
    }

    Ok(())
}