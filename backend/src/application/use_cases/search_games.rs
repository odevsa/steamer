use std::sync::Arc;

use crate::application::services::SettingsService;
use crate::domain::entities::GameSearchResult;
use crate::domain::errors::DomainError;
use crate::domain::ports::SteamGateway;

#[derive(Clone)]
pub struct SearchGamesUseCase {
    steam_gateway: Arc<dyn SteamGateway>,
    settings_service: SettingsService,
}

impl SearchGamesUseCase {
    pub fn new(steam_gateway: Arc<dyn SteamGateway>, settings_service: SettingsService) -> Self {
        Self {
            steam_gateway,
            settings_service,
        }
    }

    pub async fn execute(&self, query: &str) -> Result<Vec<GameSearchResult>, DomainError> {
        let settings = self.settings_service.get_or_init().await?;
        self.steam_gateway
            .search_games(query, &settings.steam_country_code)
            .await
    }
}
