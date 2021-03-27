extern crate json;

use std::env;

use reqwest::{Client, Method, Result};

use crate::apis::common::*;


fn get_dan_api_key() -> String {
    env::var("DAN_API_KEY").expect("To get images from danbooru you need an account (DAN_API_USERNAME) and api key (DAN_API_KEY) environment variables in .env file")
}

fn get_dan_username() -> String {
    env::var("DAN_API_USERNAME").expect("To get images from danbooru you need an account (DAN_API_USERNAME) and api key (DAN_API_KEY) environment variables in .env file.")
}

pub async fn get_posts(tags: Option<Vec<String>>) -> Result<Posts> {
    let client = Client::new();

    return if tags.is_some() {
        let tags_list = tags.unwrap().join(" ");
        let resp = client.request(Method::GET, "https://danbooru.donmai.us/posts.json")
            .basic_auth(get_dan_username(), Some(get_dan_api_key()))
            .query(&[("tags", tags_list)])
            .send()
            .await?
            .text()
            .await?;

        Ok(Posts{posts: json::parse(resp.as_str()).unwrap()})
    } else {
        let resp = client.request(Method::GET, "https://danbooru.donmai.us/posts.json")
            .basic_auth(get_dan_username(), Some(get_dan_api_key()))
            .send()
            .await?
            .text()
            .await?;


        Ok(Posts{posts: json::parse(resp.as_str()).unwrap()})
    }

}

pub async fn get_tags(starts_with: Option<String>) -> Result<Tags> {
    let client = Client::new();

    return if starts_with.is_some() {
        let starts = format!("{}{}", starts_with.unwrap(), &"*");
        let resp = client.request(Method::GET, "https://danbooru.donmai.us/tags.json")
            .basic_auth(get_dan_username(), Some(get_dan_api_key()))
            .query(&[("search[name_or_alias_matches]", starts)])
            .send()
            .await?
            .text()
            .await?;

        Ok(Tags{tags: json::parse(resp.as_str()).unwrap()})
    } else {
        let resp = client.request(Method::GET, "https://danbooru.donmai.us/tags.json")
            .basic_auth(get_dan_username(), Some(get_dan_api_key()))
            .send()
            .await?
            .text()
            .await?;

        Ok(Tags{tags: json::parse(resp.as_str()).unwrap()})
    }
}


