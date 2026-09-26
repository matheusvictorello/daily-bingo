use crate::domain::auth::Claims;
use crate::inbound::AppError;
use crate::inbound::AppState;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;
use crate::domain::user::UserId;

pub async fn authorization<S: AppState>(
    State(state): State<S>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    let claims = state
        .decode_token(&token)
        .map_err(|_| AppError::Unauthorized)?;

    let user_id: Uuid = claims
        .sub
        .parse()
        .map_err(|_| AppError::Internal("failed to parse user id from token".to_string()))?;

    req.extensions_mut().insert(UserId(user_id));

    Ok(next.run(req).await)
}
