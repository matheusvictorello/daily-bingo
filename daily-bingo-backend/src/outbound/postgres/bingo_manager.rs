use uuid::Uuid;

use crate::domain::bingo::BingoId;
use crate::domain::bingo::BingoInfo;
use crate::domain::bingo::BingoManager;
use crate::domain::common::Owned;
use crate::domain::user::UserId;

#[derive(Clone)]
pub struct PostgresBingoManager {
    db: sqlx::PgPool,
}

impl PostgresBingoManager {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    pub async fn setup(&self) -> anyhow::Result<()> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS bingos (
                id         UUID PRIMARY KEY,
                owner_id   UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                cols       INTEGER NOT NULL CHECK (cols > 0),
                rows       INTEGER NOT NULL CHECK (rows > 0),
                cells      JSONB NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now()
            )
            ",
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

impl BingoManager for PostgresBingoManager {
    async fn create_bingo(&self, bingo: Owned<UserId, BingoInfo>) -> anyhow::Result<BingoId> {
        let info = bingo.resource;
        anyhow::ensure!(
            info.values.len() == info.cols * info.rows,
            "bingo has {} cells, expected {}x{}",
            info.values.len(),
            info.cols,
            info.rows,
        );

        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO bingos (id, owner_id, cols, rows, cells) VALUES ($1, $2, $3, $4, $5::jsonb)",
        )
        .bind(id)
        .bind(bingo.owner.0)
        .bind(i32::try_from(info.cols)?)
        .bind(i32::try_from(info.rows)?)
        .bind(serde_json::to_string(&info.values)?)
        .execute(&self.db)
        .await?;

        Ok(BingoId(id))
    }
}
