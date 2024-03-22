extern crate json;

use reqwest::{Client, Result};

use tracing::*;

use crate::apis::common::Posts;


pub(crate) async fn get_posts(tags: Option<Vec<&str>>) -> Result<Posts> {
    let client = Client::new();

    return if tags.is_some() {
        let tags_list = tags.unwrap().join(" ");
        let resp = client.get("https://konachan.com/post.json")
            .query(&[("tags", tags_list)])
            .send()
            .await?
            .text()
            .await?;

        debug!(test = "Kona Post Response", response = resp.as_str());

        Ok(Posts{posts: json::parse(resp.as_str()).unwrap()})

    } else {
        let resp = client.get("https://konachan.com/post.json")
            .send()
            .await?
            .text()
            .await?;

        debug!(test = "Kona Post Response", response = resp.as_str());

        Ok(Posts{posts: json::parse(resp.as_str()).unwrap()})
    }
}