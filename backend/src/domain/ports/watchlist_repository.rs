use async_trait::async_trait;

use crate::domain::entities::{PriceHistoryPoint, WatchlistItem};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{AppId, MoneyCents};

#[async_trait]
pub trait WatchlistRepository: Send + Sync {
    async fn list_all(&self) -> Result<Vec<WatchlistItem>, DomainError>;
    async fn list_active(&self) -> Result<Vec<WatchlistItem>, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<WatchlistItem>, DomainError>;
    async fn find_by_app_id(&self, app_id: AppId) -> Result<Option<WatchlistItem>, DomainError>;
    async fn insert(&self, item: &WatchlistItem) -> Result<(), DomainError>;
    async fn update(&self, item: &WatchlistItem) -> Result<(), DomainError>;
    async fn delete(&self, id: &str) -> Result<bool, DomainError>;
}

#[async_trait]
pub trait PriceHistoryRepository: Send + Sync {
    async fn record(
        &self,
        watchlist_item_id: &str,
        price_cents: MoneyCents,
        original_price_cents: MoneyCents,
        discount_percent: u8,
    ) -> Result<(), DomainError>;

    async fn list_for_item(
        &self,
        watchlist_item_id: &str,
        limit: usize,
    ) -> Result<Vec<PriceHistoryPoint>, DomainError>;
}
