use crate::domain::common::Owned;
use crate::domain::user::UserId;

use super::BingoId;
use super::BingoInfo;

#[trait_variant::make(Send)]
pub trait BingoManager {
    async fn create_bingo(&self, bingo: Owned<UserId, BingoInfo>) -> anyhow::Result<BingoId>;
}
