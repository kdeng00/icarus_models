pub mod collection {
    use serde::{Deserialize, Serialize};
    use std::default::Default;

    use std::fs::File;
    use std::io::BufReader;

    fn is_set(num: &i32) -> bool {
        *num >= 0
    }

    pub fn parse_album(filepath: &String) -> Result<Album, serde_json::Error> {
        let file = File::open(filepath).expect("Failed to open file");
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct Album {
        #[serde(skip_serializing_if = "String::is_empty")]
        #[serde(alias = "album")]
        pub title: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        #[serde(alias = "album_artist")]
        pub artist: String,
        pub genre: String,
        pub year: i32,
        pub track_count: i32,
        #[serde(skip_serializing_if = "is_set")]
        pub disc_count: i32,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub tracks: Vec<Track>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct Track {
        pub title: String,
        pub artist: String,
        pub disc: i32,
        pub track: i32,
        // In seconds
        pub duration: f64,
    }
}
