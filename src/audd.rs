use crate::utils;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::{error::Error, io::Error as IOError, path::Path};

#[derive(Debug)]
pub struct AudD {
    api_token: String,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub images: Vec<Image>,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyData {
    pub album: Album,
    pub disc_number: u32,
    pub track_number: u32,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub album: String,
    pub artist: String,
    pub label: String,
    pub release_date: String,
    #[serde(rename(deserialize = "spotify"))]
    pub spotify_data: SpotifyData,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename(deserialize = "result"))]
    pub data: Data,
    pub status: String,
}

impl AudD {
    pub fn new(api_token: &str) -> Self {
        AudD {
            api_token: String::from(api_token),
        }
    }

    pub fn recognize(&self, path: &Path) -> Result<Response, Box<dyn Error>> {
        let data = json!({
            "api_token": self.api_token,
            "return": "spotify",
            "audio": utils::file_as_base64(path)?,
        });

        let client = Client::new();
        let response = client
            .post("https://api.audd.io/")
            .body(data.to_string())
            .send()?;

        let response: Response = response.json()?;
        if response.status != "success" {
            return Err(Box::new(IOError::new(
                std::io::ErrorKind::InvalidData,
                "unable to recognize song",
            )));
        }

        Ok(response)
    }
}
