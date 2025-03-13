use std::default::Default;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Song {
    #[serde(alias = "id")]
    pub id: Option<i32>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub duration: Option<i32>,
    pub track: Option<i32>,
    pub disc: Option<i32>,
    pub disc_count: Option<i32>,
    pub track_count: Option<i32>,
    pub audio_type: Option<String>,
    pub date_created: Option<String>,
    pub filename: Option<String>,
    pub user_id: Option<i32>,
    #[serde(skip)]
    pub data: Option<Vec<u8>>,
    #[serde(skip)]
    pub directory: Option<String>,
    #[serde(skip)]
    pub album_id: Option<i32>,
    #[serde(skip)]
    pub artist_id: Option<i32>,
    #[serde(skip)]
    pub genre_id: Option<i32>,
    #[serde(skip)]
    pub coverart_id: Option<i32>,
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: None,
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            genre: None,
            year: None,
            duration: None,
            track: None,
            disc: None,
            disc_count: None,
            track_count: None,
            audio_type: None,
            date_created: None,
            filename: None,
            user_id: None,
            data: None,
            directory: None,
            album_id: None,
            artist_id: None,
            genre_id: None,
            coverart_id: None,
        }
    }
}

impl Song {
    // TODO: Implement
    pub fn to_metadata_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }
}
