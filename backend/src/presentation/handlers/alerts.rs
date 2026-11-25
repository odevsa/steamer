use axum::extract::{Query, State};
use axum::Json;

use crate::presentation::dto::alerts::{AlertLogDto, AlertLogsQuery};
use crate::presentation::error_mapper::ApiError;
use crate::presentation::state::AppState;

pub async fn list_alerts(
    State(state): State<AppState>,
    Query(query): Query<AlertLogsQuery>,
) -> Result<Json<Vec<AlertLogDto>>, ApiError> {
    let logs = state
        .alerts
        .list_paginated(query.limit, query.offset)
        .await?;
    Ok(Json(logs.into_iter().map(Into::into).collect()))
}
