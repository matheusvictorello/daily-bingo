use axum::Router;
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::bingo::BingoCellType;
use crate::domain::bingo::BingoId;
use crate::domain::bingo::BingoInfo;
use crate::domain::common::Owned;
use crate::domain::user::UserId;
use crate::inbound::AppError;
use crate::inbound::AppState;

pub fn router<S: AppState>() -> Router<S> {
    Router::new()
        .route("/", post(create_bingo::<S>))
        .route(
            "/:bingo_id",
            get(get_bingo::<S>)
                .put(update_bingo::<S>)
                .delete(delete_bingo::<S>),
        )
}

#[derive(Serialize)]
pub struct CreateBingoResponse {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateBingoRequest {
    cols: Option<usize>,
    rows: Option<usize>,
    values: Option<Vec<BingoCellType>>,
}

async fn create_bingo<S: AppState>(
    State(state): State<S>,
    Extension(user_id): Extension<UserId>,
    Json(info): Json<BingoInfo>,
) -> Result<Json<CreateBingoResponse>, AppError> {
    info.validate()
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    let id = state
        .create_bingo(Owned::new(user_id, info))
        .await?;

    Ok(Json(CreateBingoResponse { id: id.0 }))
}

async fn get_bingo<S: AppState>(
    State(state): State<S>,
    Extension(user_id): Extension<UserId>,
    Path(bingo_id): Path<Uuid>,
) -> Result<Json<BingoInfo>, AppError> {
    let info = state
        .get_bingo(&Owned::new(user_id, BingoId(bingo_id)))
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(info))
}

async fn update_bingo<S: AppState>(
    State(state): State<S>,
    Extension(user_id): Extension<UserId>,
    Path(bingo_id): Path<Uuid>,
    Json(req): Json<UpdateBingoRequest>,
) -> Result<Json<BingoInfo>, AppError> {
    let bingo = Owned::new(user_id, BingoId(bingo_id));

    // ponytail: read-modify-write without a transaction, concurrent PUTs are last-write-wins
    let current = state
        .get_bingo(&bingo)
        .await?
        .ok_or(AppError::NotFound)?;

    let info = BingoInfo {
        cols: req.cols.unwrap_or(current.cols),
        rows: req.rows.unwrap_or(current.rows),
        values: req.values.unwrap_or(current.values),
    };

    info.validate()
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    if !state.update_bingo(&bingo, info).await? {
        return Err(AppError::NotFound);
    }

    let info = state
        .get_bingo(&bingo)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(info))
}

async fn delete_bingo<S: AppState>(
    State(state): State<S>,
    Extension(user_id): Extension<UserId>,
    Path(bingo_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if !state.delete_bingo(&Owned::new(user_id, BingoId(bingo_id))).await? {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
