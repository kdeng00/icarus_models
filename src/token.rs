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

    pub fn contains_scope(&self, des_scope: &String) -> bool {
        self.scope.contains(des_scope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_scope_check() {
        let mut token = Token::default();
        token.scope = String::from("song:read song:upload song:download");

        let check_scope = String::from("song:download");
        let result = token.contains_scope(&check_scope);

        assert!(
            result,
            "Error: The scope {:?} was not found in the token's scope {:?}",
            check_scope, token.scope
        );
    }
}
