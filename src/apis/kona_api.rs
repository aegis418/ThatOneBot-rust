use reqwest::{Client, Result, Method};
use crate::util::util;

extern crate json;

async fn get_posts(tags: Option<Vec<&str>>) -> json::JsonValue {
    let client = Client::new();

    return if tags.is_some() {
        let tags_list = tags.unwrap().join(",");
        let resp = client.get("https://konachan.com/post.json")
            .query(&[("tags", tags_list)])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        json::parse(resp.as_str()).unwrap()

    } else {
        let resp = client.get("https://konachan.com/post.json")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        json::parse(resp.as_str()).unwrap()
    }
}

fn get_random_post(posts: json::JsonValue) -> String {
    let num = util::get_rand_num(0, posts.len());
    return if !posts[num]["file_url"].is_empty() {
        posts[num]["file_url"].to_string()
    } else {
        String::from("")
    }
}