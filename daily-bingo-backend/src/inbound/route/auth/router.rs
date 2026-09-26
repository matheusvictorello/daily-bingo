use axum::Router;
use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    routing::post,
};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::inbound::AppState;

use crate::domain::common::Owned;
use crate::domain::user::UserId;
use crate::domain::user::UserInfo;
use crate::inbound::AppError;
use chrono::Utc;

pub fn router<S: AppState>() -> Router<S> {
    Router::new()
        .route("/signup", post(signup::<S>))
        .route("/login", post(login::<S>))
}

#[derive(Deserialize)]
pub struct AuthRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

async fn signup<S: AppState>(
    State(state): State<S>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let AuthRequest { email, password } = req;

    let password_hash = state
        .hash_password(&password)
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let user = Owned::new(
        UserId(Uuid::new_v4()),
        UserInfo {
            email,
            password_hash,
            created_at: Utc::now(),
        },
    );

    let token = state
        .encode_token(&user.owner)
        .map_err(|err| AppError::Internal(err.to_string()))?;

    state
        .create_user(user)
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    Ok(Json(AuthResponse { token }))
}

async fn login<S: AppState>(
    State(state): State<S>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let AuthRequest { email, password } = req;

    let user = state
        .get_user_by_email(&email)
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?
        .ok_or(AppError::Unauthorized)?;

    let valid = state
        .verify_password(&password, &user.resource.password_hash)
        .map_err(|err| AppError::Internal(err.to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let token = state
        .encode_token(&user.owner)
        .map_err(|err| AppError::Internal(err.to_string()))?;

    Ok(Json(AuthResponse { token }))
}
