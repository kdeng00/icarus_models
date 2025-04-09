use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    pub scope: String,
    pub expiration: i64,
    pub audience: String,
    pub issuer: String,
    pub issued: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessToken {
    #[serde(alias = "init::is_uuid_nil")]
    pub user_id: uuid::Uuid,
    #[serde(alias = "username")]
    pub username: String,
    #[serde(alias = "token")]
    pub token: String,
    #[serde(alias = "token_type")]
    pub token_type: String,
    #[serde(alias = "expiration")]
    pub expiration: i64,
    #[serde(alias = "message")]
    pub message: String,
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

impl AccessToken {
    pub fn bearer_token(&self) -> String {
        let mut token: String = String::from("Bearer ");
        token += &self.token.clone();
        token
    }
}

impl Token {
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }

    // TODO: Implement
    pub fn token_expired(&self) -> bool {
        false
    }

    // TODO: Implement
    pub fn contains_scope(&self, des_scope: &String) -> bool {
        let extracted_token: String = String::from("Token");

        if extracted_token == *des_scope {
            return true;
        }

        false
    }
}
