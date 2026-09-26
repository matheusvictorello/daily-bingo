use super::PasswordManager;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::domain::user::UserId;
use chrono::TimeDelta;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use super::Claims;

#[derive(Clone)]
pub struct StaticPasswordManager {
    jwt_secret: String,
    token_ttl: TimeDelta,
}

impl StaticPasswordManager {
    pub fn new(jwt_secret: String, token_ttl: TimeDelta) -> Self {
        Self {
            jwt_secret,
            token_ttl,
        }
    }
}

impl PasswordManager for StaticPasswordManager {
    fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| anyhow::anyhow!("Failed to hash password"))?;

        Ok(hash.to_string())
    }

    fn verify_password(&self, password: &str, hash: &str) -> anyhow::Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|err| anyhow::anyhow!("Failed to parse password hash"))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    fn encode_token(&self, user_id: &UserId) -> anyhow::Result<String> {
        let exp = Utc::now() + self.token_ttl;

        let claims = Claims {
            sub: user_id.0.to_string(),
            exp: exp.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    fn decode_token(&self, token: &str) -> anyhow::Result<Claims> {
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?
        .claims;

        Ok(claims)
    }
}
