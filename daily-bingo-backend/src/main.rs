#![allow(unused)]

mod inbound;
mod domain;
mod outbound;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use chrono::TimeDelta;
use std::time::Duration;
use std::sync::Arc;

use crate::domain::auth::StaticPasswordManager;
use crate::inbound::StaticAppState;
use crate::inbound::router;
use crate::outbound::postgres::PostgresBingoManager;
use crate::outbound::postgres::PostgresUserManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "daily_bingo=debug,tower_http=debug,axum=info".into()),
        )
        .init();

    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let password_manager = Arc::new(
        StaticPasswordManager::new(
            jwt_secret,
            TimeDelta::minutes(15),
        )
    );

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .acquire_timeout(Duration::new(1, 0))
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let user_manager = Arc::new(
        PostgresUserManager::new(db.clone())
    );
    user_manager
        .setup()
        .await?;

    let bingo_manager = Arc::new(
        PostgresBingoManager::new(db.clone())
    );
    bingo_manager
        .setup()
        .await?;

    let app = router(StaticAppState {
        password_manager,
        user_manager,
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind port 3000");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}
