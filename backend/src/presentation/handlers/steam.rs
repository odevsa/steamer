use axum::extract::{Query, State};
use axum::Json;

use crate::presentation::dto::watchlist::{GameSearchItemDto, SearchQuery};
use crate::presentation::error_mapper::ApiError;
use crate::presentation::state::AppState;

pub async fn search_steam_games(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<GameSearchItemDto>>, ApiError> {
    let items = state.search_games.execute(&query.q).await?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}
