use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct LoginResult {
    pub id: uuid::Uuid,
    pub username: String,
    pub token: String,
    #[serde(alias = "token_type")]
    pub token_type: String,
    pub expiration: i64,
}

impl LoginResult {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }

    pub fn token_expired(&self) -> bool {
        let current_time = time::OffsetDateTime::now_utc();
        let expired = time::OffsetDateTime::from_unix_timestamp(self.expiration).unwrap();
        current_time > expired
    }
}
