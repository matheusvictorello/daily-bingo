use super::Claims;
use crate::domain::user::UserId;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub trait PasswordManager {
    fn create_bingo(&self, bingo: Owned<UserId, BingoInfo>) -> anyhow::Result<()>;
}
