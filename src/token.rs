use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    pub scope: String,
    pub expiration: i32,
    pub audience: String,
    pub issuer: String,
    pub issued: i32,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            scope: String::new(),
            expiration: -1,
            audience: String::new(),
            issuer: String::new(),
            issued: -1,
        }
    }
}

impl Token {
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }

    // TODO: Implement
    pub fn token_expired(&self) -> bool {
        return false;
    }

    // TODO: Implement
    pub fn contains_scope(&self, des_scope: String) -> bool {
        return false;
    }
}
