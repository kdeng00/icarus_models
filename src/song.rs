use std::default::Default;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Song {
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(alias = "id")]
    pub id: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub artist: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub album: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub album_artist: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub genre: String,
    #[serde(skip_serializing_if = "is_zero")]
    pub year: i32,
    #[serde(skip_serializing_if = "is_dur_not_set")]
    pub duration: i32,
    #[serde(skip_serializing_if = "is_zero")]
    pub track: i32,
    #[serde(skip_serializing_if = "is_zero")]
    pub disc: i32,
    #[serde(skip_serializing_if = "is_zero")]
    pub disc_count: i32,
    #[serde(skip_serializing_if = "is_zero")]
    pub track_count: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub audio_type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub date_created: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub filename: String,
    #[serde(skip_serializing_if = "is_zero")]
    pub user_id: i32,
    #[serde(skip)]
    pub data: Vec<u8>,
    #[serde(skip)]
    pub directory: String,
    #[serde(skip)]
    pub album_id: i32,
    #[serde(skip)]
    pub artist_id: i32,
    #[serde(skip)]
    pub genre_id: i32,
    #[serde(skip)]
    pub coverart_id: i32,
}

fn is_zero(num: &i32) -> bool {
    *num == 0
}

fn is_dur_not_set(num: &i32) -> bool {
    *num == 0
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: 0,
            title: String::new(),
            artist: String::new(),
            album: String::new(),
            album_artist: String::new(),
            genre: String::new(),
            year: 0,
            duration: 0,
            track: 0,
            disc: 0,
            disc_count: 0,
            track_count: 0,
            audio_type: String::new(),
            date_created: String::new(),
            filename: String::new(),
            user_id: 0,
            data: Vec::new(),
            directory: String::new(),
            album_id: 0,
            artist_id: 0,
            genre_id: 0,
            coverart_id: 0,
        }
    }
}

impl Song {
    pub fn to_metadata_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            return serde_json::to_string_pretty(&self);
        } else {
            return serde_json::to_string(&self);
        }
    }

    pub fn song_path(&self) -> Result<String, std::io::Error> {
        if self.directory.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Directory does not exist",
            ));
        }

        let directory = &self.directory;
        let mut buffer: String = String::from(directory.clone());
        let last_index = directory.len() - 1;

        if let Some(character) = directory.chars().nth(last_index) {
            if character != '/' {
                buffer += "/";
            }

            buffer += &self.filename.clone();

            return Ok(buffer);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not access last character of directory",
            ));
        }
    }

    pub fn to_data(&self) -> Result<Vec<u8>, std::io::Error> {
        let path_result = self.song_path();

        match path_result {
            Ok(path) => {
                let mut file = std::fs::File::open(path)?;
                let mut buffer: Vec<u8> = Vec::new();
                file.read_to_end(&mut buffer)?;

                if buffer.len() == 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "File is empty",
                    ));
                } else {
                    return Ok(buffer);
                }
            }
            Err(er) => {
                return Err(er);
            }
        }
    }
}

mod embedded {
    use std::io::Read;

    use serde::{Deserialize, Serialize};

    impl Song {
        pub fn to_metadata_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
            if pretty {
                return serde_json::to_string_pretty(&self);
            } else {
                return serde_json::to_string(&self);
            }
        }

        pub fn song_path(&self) -> Result<String, std::io::Error> {
            if self.directory.is_empty() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Directory does not exist",
                ));
            }

            let directory = &self.directory;
            let mut buffer: String = String::from(directory.clone());
            let last_index = directory.len() - 1;

            if let Some(character) = directory.chars().nth(last_index) {
                if character != '/' {
                    buffer += "/";
                }

                buffer += &self.filename.clone();

                return Ok(buffer);
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Could not access last character of directory",
                ));
            }
        }

        pub fn to_data(&self) -> Result<Vec<u8>, std::io::Error> {
            let path_result = self.song_path();

            match path_result {
                Ok(path) => {
                    let mut file = std::fs::File::open(path)?;
                    let mut buffer: Vec<u8> = Vec::new();
                    file.read_to_end(&mut buffer)?;

                    if buffer.len() == 0 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "File is empty",
                        ));
                    } else {
                        return Ok(buffer);
                    }
                }
                Err(er) => {
                    return Err(er);
                }
            }
        }
    }

    // The song's duration is a floating point in seconds
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Song {
        #[serde(skip_serializing_if = "is_embed_zero")]
        #[serde(alias = "id")]
        pub id: i32,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub title: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub artist: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub album: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub album_artist: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub genre: String,
        #[serde(skip_serializing_if = "is_embed_zero")]
        pub year: i32,
        #[serde(skip_serializing_if = "is_embed_dur_not_set")]
        pub duration: f64,
        #[serde(skip_serializing_if = "is_embed_zero")]
        pub track: i32,
        #[serde(skip_serializing_if = "is_embed_zero")]
        pub disc: i32,
        #[serde(skip_serializing_if = "is_embed_zero")]
        pub disc_count: i32,
        #[serde(skip_serializing_if = "is_embed_zero")]
        pub track_count: i32,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub audio_type: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub date_created: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub filename: String,
        #[serde(skip_serializing_if = "is_embed_zero")]
        pub user_id: i32,
        #[serde(skip)]
        pub data: Vec<u8>,
        #[serde(skip)]
        pub directory: String,
        #[serde(skip)]
        pub album_id: i32,
        #[serde(skip)]
        pub artist_id: i32,
        #[serde(skip)]
        pub genre_id: i32,
        #[serde(skip)]
        pub coverart_id: i32,
    }

    fn is_embed_zero(num: &i32) -> bool {
        *num == 0
    }

    fn is_embed_dur_not_set(num: &f64) -> bool {
        *num == 0.0
    }

    impl Default for Song {
        fn default() -> Self {
            Song {
                id: 0,
                title: String::new(),
                artist: String::new(),
                album: String::new(),
                album_artist: String::new(),
                genre: String::new(),
                year: 0,
                duration: 0.0,
                track: 0,
                disc: 0,
                disc_count: 0,
                track_count: 0,
                audio_type: String::new(),
                date_created: String::new(),
                filename: String::new(),
                user_id: 0,
                data: Vec::new(),
                directory: String::new(),
                album_id: 0,
                artist_id: 0,
                genre_id: 0,
                coverart_id: 0,
            }
        }
    }
}
