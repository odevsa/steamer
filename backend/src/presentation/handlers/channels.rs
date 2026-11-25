use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use crate::presentation::dto::channels::{
    ChannelDto, CreateChannelRequest, TestChannelResponse, UpdateChannelRequest,
};
use crate::presentation::error_mapper::ApiError;
use crate::presentation::state::AppState;

pub async fn list_channels(
    State(state): State<AppState>,
) -> Result<Json<Vec<ChannelDto>>, ApiError> {
    let channels = state.channels.list_all().await?;
    Ok(Json(channels.into_iter().map(Into::into).collect()))
}

pub async fn create_channel(
    State(state): State<AppState>,
    Json(payload): Json<CreateChannelRequest>,
) -> Result<(StatusCode, Json<ChannelDto>), ApiError> {
    let channel = state
        .channels
        .create_channel(payload.name, payload.config.into(), payload.is_enabled)
        .await?;
    Ok((StatusCode::CREATED, Json(channel.into())))
}

pub async fn update_channel(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateChannelRequest>,
) -> Result<Json<ChannelDto>, ApiError> {
    let channel = state
        .channels
        .update_channel(
            &id,
            payload.name,
            payload.config.map(Into::into),
            payload.is_enabled,
        )
        .await?;
    Ok(Json(channel.into()))
}

pub async fn delete_channel(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    state.channels.delete_channel(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn test_channel(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TestChannelResponse>, ApiError> {
    state.channels.test_channel(&id).await?;
    Ok(Json(TestChannelResponse { status: "sent" }))
}
