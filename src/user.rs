use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(skip_serializing_if = "is_zero")]
    pub id: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub username: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub password: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub email: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub phone: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub firstname: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lastname: String,
    pub email_verified: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub date_created: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub last_login: String,
}

fn is_zero(num: &i32) -> bool {
    *num == 0
}

impl Default for User {
    fn default() -> Self {
        User {
            id: -1,
            username: String::new(),
            password: String::new(),
            email: String::new(),
            phone: String::new(),
            firstname: String::new(),
            lastname: String::new(),
            email_verified: false,
            date_created: String::new(),
            status: String::new(),
            last_login: String::new(),
        }
    }
}

impl User {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }
}
