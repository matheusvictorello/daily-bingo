use std::sync::Arc;

use crate::domain::auth::*;
use crate::domain::bingo::*;
use crate::domain::common::*;
use crate::domain::user::*;

pub trait AppState:
    PasswordManager
    + UserManager
    + BingoManager
    + Clone
    + Send
    + Sync
    + 'static
{}

#[derive(Clone)]
pub struct StaticAppState<Pa, Us, Bi> {
    pub password_manager: Arc<Pa>,
    pub user_manager: Arc<Us>,
    pub bingo_manager: Arc<Bi>,
}

impl<Pa, Us, Bi> AppState for StaticAppState<Pa, Us, Bi>
where
    Pa: PasswordManager + Clone + Send + Sync + 'static,
    Us: UserManager + Clone + Send + Sync + 'static,
    Bi: BingoManager + Clone + Send + Sync + 'static,
{}

impl<Pa, Us, Bi> PasswordManager for StaticAppState<Pa, Us, Bi>
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

impl<Pa, Us, Bi> UserManager for StaticAppState<Pa, Us, Bi>
where
    Pa: PasswordManager + Clone + Send + Sync + 'static,
    Us: UserManager + Clone + Send + Sync + 'static,
    Bi: Send + Sync,
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

impl<Pa, Us, Bi> BingoManager for StaticAppState<Pa, Us, Bi>
where
    Pa: Send + Sync,
    Us: Send + Sync,
    Bi: BingoManager + Clone + Send + Sync + 'static,
{
    async fn create_bingo(&self, bingo: Owned<UserId, BingoInfo>) -> anyhow::Result<BingoId> {
        self.bingo_manager.create_bingo(bingo).await
    }

    async fn list_bingos(&self, owner: &UserId) -> anyhow::Result<Vec<(BingoId, BingoInfo)>> {
        self.bingo_manager.list_bingos(owner).await
    }

    async fn get_bingo(&self, bingo: &Owned<UserId, BingoId>) -> anyhow::Result<Option<BingoInfo>> {
        self.bingo_manager.get_bingo(bingo).await
    }

    async fn update_bingo(
        &self,
        bingo: &Owned<UserId, BingoId>,
        info: BingoInfo,
    ) -> anyhow::Result<bool> {
        self.bingo_manager.update_bingo(bingo, info).await
    }

    async fn delete_bingo(&self, bingo: &Owned<UserId, BingoId>) -> anyhow::Result<bool> {
        self.bingo_manager.delete_bingo(bingo).await
    }
}
