use reqwest::{Client, Result, Method, Error};
use crate::util::util;

extern crate json;

async fn get_posts(tags: Option<Vec<&str>>) -> Result<json::JsonValue> {
    let client = Client::new();

    return if tags.is_some() {
        let tags_list = tags.unwrap().join(",");
        let resp = client.request(Method::GET, "https://yande.re/post.json")
            .query(&[("tags", tags_list)])
            .send()
            .await?
            .text()
            .await?;

        Ok(json::parse(resp.as_str()).unwrap())
    } else {
        let resp = client.request(Method::GET, "https://yande.re.post.json")
            .send()
            .await?
            .text()
            .await?;

        Ok(json::parse(resp.as_str()).unwrap())
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