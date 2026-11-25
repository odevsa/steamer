use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::application::services::{PriceMonitorWorker, SettingsService};
use crate::domain::entities::{PriceHistoryPoint, WatchlistItem};
use crate::domain::errors::DomainError;
use crate::domain::ports::{PriceHistoryRepository, SteamGateway, WatchlistRepository};
use crate::domain::value_objects::{AppId, MoneyCents};

const MANUAL_CHECK_COOLDOWN: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub struct WatchlistUseCase {
    watchlist_repo: Arc<dyn WatchlistRepository>,
    price_history_repo: Arc<dyn PriceHistoryRepository>,
    steam_gateway: Arc<dyn SteamGateway>,
    settings_service: SettingsService,
    worker: PriceMonitorWorker,
    last_manual_check: Arc<Mutex<Option<Instant>>>,
}

impl WatchlistUseCase {
    pub fn new(
        watchlist_repo: Arc<dyn WatchlistRepository>,
        price_history_repo: Arc<dyn PriceHistoryRepository>,
        steam_gateway: Arc<dyn SteamGateway>,
        settings_service: SettingsService,
        worker: PriceMonitorWorker,
    ) -> Self {
        Self {
            watchlist_repo,
            price_history_repo,
            steam_gateway,
            settings_service,
            worker,
            last_manual_check: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn list_all(&self) -> Result<Vec<WatchlistItem>, DomainError> {
        self.watchlist_repo.list_all().await
    }

    pub async fn add_item(
        &self,
        raw_app_id: i64,
        raw_target_price_cents: Option<i64>,
    ) -> Result<WatchlistItem, DomainError> {
        let app_id = AppId::new(raw_app_id)?;
        let target_price_cents = raw_target_price_cents.map(MoneyCents::new).transpose()?;

        if self.watchlist_repo.find_by_app_id(app_id).await?.is_some() {
            return Err(DomainError::Conflict(
                "game is already in watchlist".to_string(),
            ));
        }

        let settings = self.settings_service.get_or_init().await?;
        let snapshot = self
            .steam_gateway
            .fetch_app_price(app_id, &settings.steam_country_code)
            .await?;

        let now = Utc::now();
        let item = WatchlistItem {
            id: Uuid::new_v4().to_string(),
            app_id,
            title: snapshot.title,
            header_image_url: snapshot.header_image_url,
            target_price_cents,
            currency: snapshot.currency,
            current_price_cents: Some(snapshot.current_price_cents),
            original_price_cents: Some(snapshot.original_price_cents),
            discount_percent: snapshot.discount_percent,
            is_free: snapshot.is_free,
            is_active: true,
            last_notified_price_cents: None,
            last_notified_at: None,
            last_checked_at: Some(snapshot.fetched_at),
            created_at: now,
        };

        self.watchlist_repo.insert(&item).await?;
        self.price_history_repo
            .record(
                &item.id,
                snapshot.current_price_cents,
                snapshot.original_price_cents,
                snapshot.discount_percent,
            )
            .await?;

        Ok(item)
    }

    pub async fn update_item(
        &self,
        id: &str,
        target_price_cents: Option<Option<i64>>,
        is_active: Option<bool>,
    ) -> Result<WatchlistItem, DomainError> {
        let mut item = self
            .watchlist_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("watchlist item not found".to_string()))?;

        if let Some(target_update) = target_price_cents {
            let parsed = target_update.map(MoneyCents::new).transpose()?;
            if parsed != item.target_price_cents {
                item.target_price_cents = parsed;
                item.last_notified_price_cents = None;
            }
        }

        if let Some(active) = is_active {
            item.is_active = active;
        }

        self.watchlist_repo.update(&item).await?;
        Ok(item)
    }

    pub async fn delete_item(&self, id: &str) -> Result<(), DomainError> {
        if !self.watchlist_repo.delete(id).await? {
            return Err(DomainError::NotFound(
                "watchlist item not found".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn list_history(
        &self,
        id: &str,
        limit: usize,
    ) -> Result<Vec<PriceHistoryPoint>, DomainError> {
        if self.watchlist_repo.find_by_id(id).await?.is_none() {
            return Err(DomainError::NotFound(
                "watchlist item not found".to_string(),
            ));
        }
        self.price_history_repo.list_for_item(id, limit).await
    }

    pub async fn trigger_manual_check(&self) -> Result<(), DomainError> {
        let mut guard = self.last_manual_check.lock().await;
        if let Some(last) = *guard {
            let elapsed = last.elapsed();
            if elapsed < MANUAL_CHECK_COOLDOWN {
                let remaining = (MANUAL_CHECK_COOLDOWN - elapsed).as_secs().max(1);
                return Err(DomainError::RateLimited(format!(
                    "manual price check cooldown active, retry in {remaining}s"
                )));
            }
        }
        *guard = Some(Instant::now());
        drop(guard);

        self.worker.trigger_now();
        Ok(())
    }
}
