use std::sync::Arc;

use crate::domain::auth::*;
use crate::domain::common::*;
use crate::domain::user::*;

pub trait AppState:
    PasswordManager
    + UserManager
    + Clone
    + Send
    + Sync
    + 'static
{}

#[derive(Clone)]
pub struct StaticAppState<Pa, Us> {
    pub password_manager: Arc<Pa>,
    pub user_manager: Arc<Us>,
}

impl<Pa, Us> AppState for StaticAppState<Pa, Us>
where
    Pa: PasswordManager + Clone + Send + Sync + 'static,
    Us: UserManager + Clone + Send + Sync + 'static,
{}

impl<Pa, Us> PasswordManager for StaticAppState<Pa, Us>
where
    Pa: PasswordManager + Clone + Send + Sync + 'static,
{
    fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        self.password_manager.hash_password(password)
    }

    fn verify_password(&self, password: &str, hash: &str) -> anyhow::Result<bool> {
        self.password_manager.verify_password(password, hash)
    }

    fn encode_token(&self, user_id: &UserId) -> anyhow::Result<String> {
        self.password_manager.encode_token(user_id)
    }

    fn decode_token(&self, token: &str) -> anyhow::Result<Claims> {
        self.password_manager.decode_token(token)
    }
}

impl<Pa, Us> UserManager for StaticAppState<Pa, Us>
where
    Pa: PasswordManager + Clone + Send + Sync + 'static,
    Us: UserManager + Clone + Send + Sync + 'static,
{
    async fn create_user(&self, user: Owned<UserId, UserInfo>) -> anyhow::Result<()> {
        self.user_manager.create_user(user).await
    }

    async fn get_user_info(&self, user_id: &UserId) -> anyhow::Result<Option<UserInfo>> {
        self.user_manager.get_user_info(user_id).await
    }

    async fn get_user_by_email(
        &self,
        email: &str,
    ) -> anyhow::Result<Option<Owned<UserId, UserInfo>>> {
        self.user_manager.get_user_by_email(email).await
    }
}
