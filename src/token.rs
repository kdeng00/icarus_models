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

pub fn get_issued() -> time::Result<time::OffsetDateTime> {
    Ok(time::OffsetDateTime::now_utc())
}

pub fn get_expiration(issued: &time::OffsetDateTime) -> Result<time::OffsetDateTime, time::Error> {
    let duration_expire = time::Duration::hours(4);
    Ok(*issued + duration_expire)
}

mod util {
    pub fn time_to_std_time(
        provided_time: &time::OffsetDateTime,
    ) -> Result<std::time::SystemTime, std::time::SystemTimeError> {
        let converted = std::time::SystemTime::from(*provided_time);
        Ok(converted)
    }
}

pub fn create_token(
    key: &String,
    message: &String,
    issuer: &String,
    audience: &String,
) -> Result<(String, i64), josekit::JoseError> {
    let mut header = josekit::jws::JwsHeader::new();
    header.set_token_type("JWT");

    let mut payload = josekit::jwt::JwtPayload::new();
    payload.set_subject(message);
    payload.set_issuer(issuer);
    payload.set_audience(vec![audience]);
    match get_issued() {
        Ok(issued) => {
            let expire = get_expiration(&issued).unwrap();
            payload.set_issued_at(&util::time_to_std_time(&issued).unwrap());
            payload.set_expires_at(&util::time_to_std_time(&expire).unwrap());

            let signer = josekit::jws::alg::hmac::HmacJwsAlgorithm::Hs256
                .signer_from_bytes(key.as_bytes())
                .unwrap();
            Ok((
                josekit::jwt::encode_with_signer(&payload, &header, &signer).unwrap(),
                (expire - time::OffsetDateTime::UNIX_EPOCH).whole_seconds(),
            ))
        }
        Err(e) => Err(josekit::JoseError::InvalidClaim(e.into())),
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
