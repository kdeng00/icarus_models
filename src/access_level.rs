use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessLevel {
    pub id: i32,
    pub level: String,
    pub song_id: i32,
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel {
            id: -1,
            level: String::new(),
            song_id: -1,
        }
    }
}

impl AccessLevel {
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }
}
