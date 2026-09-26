use crate::domain::common::Owned;
use crate::domain::user::UserId;

use super::BingoId;
use super::BingoInfo;

#[trait_variant::make(Send)]
pub trait BingoManager {
    async fn create_bingo(&self, bingo: Owned<UserId, BingoInfo>) -> anyhow::Result<BingoId>;

    async fn get_bingo(&self, bingo: &Owned<UserId, BingoId>) -> anyhow::Result<Option<BingoInfo>>;

    /// Returns false if the bingo doesn't exist or isn't owned by the user.
    async fn update_bingo(
        &self,
        bingo: &Owned<UserId, BingoId>,
        info: BingoInfo,
    ) -> anyhow::Result<bool>;

    /// Returns false if the bingo doesn't exist or isn't owned by the user.
    async fn delete_bingo(&self, bingo: &Owned<UserId, BingoId>) -> anyhow::Result<bool>;
}
