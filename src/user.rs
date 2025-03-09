use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub firstname: String,
    pub lastname: String,
    pub email_verified: bool,
    pub date_created: String,
    pub status: String,
    pub last_login: String,
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
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string_pretty(&self);
    }
}
