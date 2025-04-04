use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessLevel {
    pub id: uuid::Uuid,
    pub level: String,
    pub song_id: uuid::Uuid,
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel {
            id: uuid::Uuid::new_v4(),
            level: String::new(),
            song_id: uuid::Uuid::new_v4(),
        }
    }
}

pub fn default_level() -> AccessLevel {
    AccessLevel {
        id: uuid::Uuid::new_v4(),
        level: String::from("Public"),
        song_id: uuid::Uuid::new_v4(),
    }
}

pub fn private_level() -> AccessLevel {
    AccessLevel {
        id: uuid::Uuid::new_v4(),
        level: String::from("Private"),
        song_id: uuid::Uuid::new_v4(),
    }
}

impl AccessLevel {
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
