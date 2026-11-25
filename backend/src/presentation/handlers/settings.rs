use axum::extract::State;
use axum::Json;

use crate::presentation::dto::settings::{SettingsResponse, UpdateSettingsRequest};
use crate::presentation::error_mapper::ApiError;
use crate::presentation::state::AppState;

pub async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<SettingsResponse>, ApiError> {
    let view = state.settings.get().await?;
    Ok(Json(view.into()))
}

pub async fn update_settings(
    State(state): State<AppState>,
    Json(payload): Json<UpdateSettingsRequest>,
) -> Result<Json<SettingsResponse>, ApiError> {
    let view = state
        .settings
        .update(
            payload.steam_country_code,
            payload.ui_locale,
            payload.poll_interval_secs,
        )
        .await?;
    Ok(Json(view.into()))
}
