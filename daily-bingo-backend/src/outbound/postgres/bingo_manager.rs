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

    async fn get_bingo(&self, bingo: &Owned<UserId, BingoId>) -> anyhow::Result<Option<BingoInfo>> {
        let row = sqlx::query_as::<_, (i32, i32, String)>(
            "SELECT cols, rows, cells::text FROM bingos WHERE id = $1 AND owner_id = $2",
        )
        .bind(bingo.resource.0)
        .bind(bingo.owner.0)
        .fetch_optional(&self.db)
        .await?;

        row.map(|(cols, rows, cells)| {
            Ok(BingoInfo {
                cols: cols.try_into()?,
                rows: rows.try_into()?,
                values: serde_json::from_str(&cells)?,
            })
        })
        .transpose()
    }

    async fn update_bingo(
        &self,
        bingo: &Owned<UserId, BingoId>,
        info: BingoInfo,
    ) -> anyhow::Result<bool> {
        let result = sqlx::query(
            "UPDATE bingos SET cols = $3, rows = $4, cells = $5::jsonb WHERE id = $1 AND owner_id = $2",
        )
        .bind(bingo.resource.0)
        .bind(bingo.owner.0)
        .bind(i32::try_from(info.cols)?)
        .bind(i32::try_from(info.rows)?)
        .bind(serde_json::to_string(&info.values)?)
        .execute(&self.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_bingo(&self, bingo: &Owned<UserId, BingoId>) -> anyhow::Result<bool> {
        let result = sqlx::query("DELETE FROM bingos WHERE id = $1 AND owner_id = $2")
            .bind(bingo.resource.0)
            .bind(bingo.owner.0)
            .execute(&self.db)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
