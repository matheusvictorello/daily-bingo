use crate::domain::common::Owned;

use super::UserId;
use super::UserInfo;

#[trait_variant::make(Send)]
pub trait UserManager {
    async fn create_user(&self, user: Owned<UserId, UserInfo>) -> anyhow::Result<()>;

    async fn get_user_info(&self, user_id: &UserId) -> anyhow::Result<Option<UserInfo>>;

    async fn get_user_by_email(
        &self,
        email: &str,
    ) -> anyhow::Result<Option<Owned<UserId, UserInfo>>>;
}
