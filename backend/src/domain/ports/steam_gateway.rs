use async_trait::async_trait;

use crate::domain::entities::{GamePriceSnapshot, GameSearchResult};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{AppId, CountryCode};

#[async_trait]
pub trait SteamGateway: Send + Sync {
    async fn search_games(
        &self,
        query: &str,
        country_code: &CountryCode,
    ) -> Result<Vec<GameSearchResult>, DomainError>;

    async fn fetch_app_price(
        &self,
        app_id: AppId,
        country_code: &CountryCode,
    ) -> Result<GamePriceSnapshot, DomainError>;
}
