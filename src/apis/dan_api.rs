use std::env;
use reqwest::{Client, Result, Method};
use crate::util::util;

extern crate json;

fn get_dan_api_key() -> String {
    env::var("DAN_API_KEY").expect("To get images from danbooru you need an account (DAN_API_USERNAME) and api key (DAN_API_KEY) environment variables in .env file")
}

fn get_dan_username() -> String {
    env::var("DAN_API_USERNAME").expect("To get images from danbooru you need an account (DAN_API_USERNAME) and api key (DAN_API_KEY) environment variables in .env file.")
}

async fn get_posts(tags: Option<Vec<&str>>) -> Result<json::JsonValue> {
    let client = Client::new();

    return if tags.is_some() {
        let tags_list = tags.unwrap().join(",");
        let resp = client.request(Method::GET, "https://danbooru.donmai.us/posts.json")
            .basic_auth(get_dan_username(), Some(get_dan_api_key()))
            .query(&[("tags", tags_list)])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        Ok(json::parse(resp.as_str()).unwrap())
    } else {
        let resp = client.request(Method::GET, "https://danbooru.donmai.us/posts.json")
            .basic_auth(get_dan_username(), Some(get_dan_api_key()))
            .send()
            .await?
            .text()
            .await?;


        Ok(json::parse(resp.as_str()).unwrap())
    }

}

async fn get_tags(starts_with: Option<&str>) -> json::JsonValue {
    let client = Client::new();

    return if starts_with.is_some() {
        let starts = format!("{}{}", starts_with.unwrap(), &"*");
        let resp = client.request(Method::GET, "https://danbooru.donmai.us/tags.json")
            .basic_auth(get_dan_username(), Some(get_dan_api_key()))
            .query(&[("search[name_or_alias_matches]", starts)])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let json = json::parse(resp.as_str()).unwrap();
        json
    } else {
        let resp = client.request(Method::GET, "https://danbooru.donmai.us/tags.json")
            .basic_auth(get_dan_username(), Some(get_dan_api_key()))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let json = json::parse(resp.as_str()).unwrap();
        json
    }
}

fn get_random_tag(tag_list: json::JsonValue) -> String {
    let num = util::get_rand_num(0, tag_list.len());
    tag_list[num]["name"].to_string()
}

fn get_random_post(posts: json::JsonValue) -> String {
    let num = util::get_rand_num(0, posts.len());
    return if !posts[num]["large_file_url"].is_empty() {
        posts[num]["large_file_url"].to_string()
    } else if !posts[num]["file_url"].is_empty() {
        posts[num]["file_url"].to_string()
    } else {
        String::from("")
    }
}
