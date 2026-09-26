use uuid::Uuid;

use crate::domain::common::Owned;
use crate::domain::user::UserId;
use crate::domain::user::UserInfo;
use crate::domain::user::UserManager;

#[derive(Clone)]
pub struct PostgresUserManager {
    db: sqlx::PgPool,
}

impl PostgresUserManager {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    pub async fn setup(&self) -> anyhow::Result<()> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS users (
                id            UUID PRIMARY KEY,
                email         TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
            )
            ",
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

impl UserManager for PostgresUserManager {
    async fn create_user(&self, user: Owned<UserId, UserInfo>) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(user.owner.0)
        .bind(user.resource.email)
        .bind(user.resource.password_hash)
        .bind(user.resource.created_at)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn get_user_info(&self, owner: &UserId) -> anyhow::Result<Option<UserInfo>> {
        let row = sqlx::query_as::<_, (String, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT email, password_hash, created_at FROM users WHERE id = $1",
        )
        .bind(owner.0)
        .fetch_optional(&self.db)
        .await?;

        Ok(row.map(|(email, password_hash, created_at)| UserInfo {
            email,
            password_hash,
            created_at,
        }))
    }

    async fn get_user_by_email(
        &self,
        email: &str,
    ) -> anyhow::Result<Option<Owned<UserId, UserInfo>>> {
        let row = sqlx::query_as::<_, (Uuid, String, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, email, password_hash, created_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await?;

        Ok(row.map(|(id, email, password_hash, created_at)| {
            Owned::new(
                UserId(id),
                UserInfo {
                    email,
                    password_hash,
                    created_at,
                },
            )
        }))
    }
}
