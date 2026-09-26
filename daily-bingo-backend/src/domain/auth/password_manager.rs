use super::Claims;
use crate::domain::user::UserId;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub trait PasswordManager {
    fn hash_password(&self, password: &str) -> anyhow::Result<String>;

    fn verify_password(&self, password: &str, hash: &str) -> anyhow::Result<bool>;

    fn encode_token(&self, user_id: &UserId) -> anyhow::Result<String>;

    fn decode_token(&self, token: &str) -> anyhow::Result<Claims>;
}
