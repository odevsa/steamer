use sqlx::SqlitePool;

use crate::application::use_cases::{
    AlertsUseCase, ChannelsUseCase, SearchGamesUseCase, SettingsUseCase, WatchlistUseCase,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub search_games: SearchGamesUseCase,
    pub watchlist: WatchlistUseCase,
    pub channels: ChannelsUseCase,
    pub alerts: AlertsUseCase,
    pub settings: SettingsUseCase,
}
