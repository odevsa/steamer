use axum::extract::State;
use axum::Json;

use crate::domain::errors::DomainError;
use crate::presentation::dto::common::HealthResponse;
use crate::presentation::error_mapper::ApiError;
use crate::presentation::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> Result<Json<HealthResponse>, ApiError> {
    sqlx::query("SELECT 1")
        .execute(&state.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("health check query failed: {e}")))?;

    Ok(Json(HealthResponse {
        status: "ok",
        database: "ok",
    }))
}
