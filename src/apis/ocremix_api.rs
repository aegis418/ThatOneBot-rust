use reqwest::{Client, Request, Result};
use std::fmt::Error;
use serenity::futures::future::ok;

extern crate json;

pub struct OCRemix {
    station_id: StationID,
    url: Option<String>,
    title: String,
    album: String,
    album_url: String
}

impl OCRemix {
    fn new(station_id: StationID, url: Option<String>, title: String, album: String, album_url: String) -> OCRemix {
        OCRemix {
            station_id,
            url,
            title,
            album,
            album_url
        }
    }
}

pub enum StationID {
    Game,
    OCR,
    Covers,
    Chiptunes,
    All
}

impl StationID {
    pub fn value(&self) -> u8 {
        match *self {
            StationID::Game => 1,
            StationID::OCR => 2,
            StationID::Covers => 3,
            StationID::Chiptunes => 4,
            StationID::All => 5
        }
    }

    pub async fn get_stream_url(&self) -> String {
        let client = Client::new();
        let resp = client.get("https://rainwave.cc/api4/stations")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let json = json::parse(resp.as_str()).unwrap();
        json["stations"][(self.value()-1) as usize]["stream"].to_string()
    }
}

impl Default for StationID {
    fn default() -> Self {
        StationID::All
    }
}

impl From<String> for StationID {
    fn from(id_string: String) -> Self {
        match &*id_string {
            "game" =>  StationID::Game,
            "ocr" => StationID::OCR,
            "covers" => StationID::Covers,
            "chiptunes" => StationID::Chiptunes,
            "all" => StationID::All,
            _ => StationID::default()
        }
    }
}

pub async fn get_current_song(sid: StationID) -> Result<OCRemix> {
    let base = "https://rainwave.cc";
    let client = Client::new();

    let resp = client.get("https://rainwave.cc/api4/info")
        .query(&[("sid", sid.value())])
        .send()
        .await?
        .text()
        .await?;

    let json = json::parse(&resp).unwrap();
    let title = json["sched_current"]["songs"][0]["title"].to_string();
    let album = json["sched_current"]["songs"][0]["albums"][0]["name"].to_string();
    let album_url = format!("{}{}_240.jpg", base, json["sched_current"]["songs"][0]["albums"][0]["art"].to_string());
    let url = if !json["sched_current"]["songs"][0]["url"].is_empty() {
        Some(json["sched_current"]["songs"][0]["url"].to_string())
    } else {
        None
    };

    Ok(OCRemix::new(sid, url, title, album, album_url))
}