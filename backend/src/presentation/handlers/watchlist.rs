use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::presentation::dto::watchlist::{
    AddWatchlistRequest, PriceHistoryPointDto, TriggerCheckResponse, UpdateWatchlistRequest,
    WatchlistItemDto,
};
use crate::presentation::error_mapper::ApiError;
use crate::presentation::state::AppState;

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    #[serde(default = "default_history_limit")]
    pub limit: usize,
}

fn default_history_limit() -> usize {
    100
}

pub async fn list_watchlist(
    State(state): State<AppState>,
) -> Result<Json<Vec<WatchlistItemDto>>, ApiError> {
    let items = state.watchlist.list_all().await?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn add_watchlist_item(
    State(state): State<AppState>,
    Json(payload): Json<AddWatchlistRequest>,
) -> Result<(StatusCode, Json<WatchlistItemDto>), ApiError> {
    let item = state
        .watchlist
        .add_item(payload.app_id, payload.target_price_cents)
        .await?;
    Ok((StatusCode::CREATED, Json(item.into())))
}

pub async fn update_watchlist_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateWatchlistRequest>,
) -> Result<Json<WatchlistItemDto>, ApiError> {
    let item = state
        .watchlist
        .update_item(&id, payload.target_price_cents, payload.is_active)
        .await?;
    Ok(Json(item.into()))
}

pub async fn delete_watchlist_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    state.watchlist.delete_item(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn trigger_watchlist_check(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<TriggerCheckResponse>), ApiError> {
    state.watchlist.trigger_manual_check().await?;
    Ok((
        StatusCode::ACCEPTED,
        Json(TriggerCheckResponse {
            status: "scheduled",
        }),
    ))
}

pub async fn list_watchlist_history(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<PriceHistoryPointDto>>, ApiError> {
    let points = state.watchlist.list_history(&id, query.limit).await?;
    Ok(Json(points.into_iter().map(Into::into).collect()))
}
