extern crate json;

use reqwest::{Client, Method, Result};

use crate::apis::common::Posts;


pub async fn get_posts(tags: Option<Vec<String>>) -> Result<Posts> {
    let client = Client::new();

    return if tags.is_some() {
        let tags_list = tags.unwrap().join(" ");
        let resp = client.request(Method::GET, "https://yande.re/post.json")
            .query(&[("tags", tags_list)])
            .send()
            .await?
            .text()
            .await?;

        Ok(Posts{posts: json::parse(resp.as_str()).unwrap()})
    } else {
        let resp = client.request(Method::GET, "https://yande.re/post.json")
            .send()
            .await?
            .text()
            .await?;

        Ok(Posts{posts: json::parse(resp.as_str()).unwrap()})
    }
}
