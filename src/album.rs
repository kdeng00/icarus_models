pub mod collection {
    use serde::{Deserialize, Serialize};

    fn is_set(num: &i32) -> bool {
        *num >= 0
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Album {
        #[serde(skip_serializing_if = "String::is_empty")]
        #[serde(alias = "album")]
        pub title: String,
        pub genre: String,
        pub year: i32,
        pub track_count: i32,
        #[serde(skip_serializing_if = "is_set")]
        pub disc_count: i32,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub tracks: Vec<Track>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Track {
        pub title: String,
        pub artist: String,
        pub disc: i32,
        pub track: i32,
        // In seconds
        pub duration: f64,
    }
}
