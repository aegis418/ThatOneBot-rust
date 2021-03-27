extern crate json;

use reqwest::{Client, Result};

use crate::apis::common::Posts;


pub(crate) async fn get_posts(tags: Option<Vec<String>>) -> Result<Posts> {
    let client = Client::new();

    return if tags.is_some() {
        let tags_list = tags.unwrap().join(" ");
        let resp = client.get("https://konachan.com/post.json")
            .query(&[("tags", tags_list)])
            .send()
            .await?
            .text()
            .await?;

        Ok(Posts{posts: json::parse(resp.as_str()).unwrap()})

    } else {
        let resp = client.get("https://konachan.com/post.json")
            .send()
            .await?
            .text()
            .await?;

        Ok(Posts{posts: json::parse(resp.as_str()).unwrap()})
    }
}