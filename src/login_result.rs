use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginResult {
    pub id: uuid::Uuid,
    pub username: String,
    pub token: String,
    #[serde(alias = "token_type")]
    pub token_type: String,
    pub expiration: i32,
}

impl Default for LoginResult {
    fn default() -> Self {
        LoginResult {
            id: uuid::Uuid::new_v4(),
            username: String::new(),
            token: String::new(),
            token_type: String::new(),
            expiration: -1,
        }
    }
}

impl LoginResult {
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
