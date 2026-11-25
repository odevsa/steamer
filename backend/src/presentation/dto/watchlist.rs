use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::domain::entities::{GameSearchResult, PriceHistoryPoint, WatchlistItem};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Debug, Serialize)]
pub struct GameSearchItemDto {
    pub app_id: i64,
    pub title: String,
    pub header_image_url: String,
    pub currency: Option<String>,
    pub current_price_cents: Option<i64>,
    pub original_price_cents: Option<i64>,
    pub discount_percent: u8,
    pub is_free: bool,
}

impl From<GameSearchResult> for GameSearchItemDto {
    fn from(value: GameSearchResult) -> Self {
        Self {
            app_id: value.app_id.value(),
            title: value.title,
            header_image_url: value.header_image_url,
            currency: value.currency,
            current_price_cents: value.current_price_cents.map(|m| m.value()),
            original_price_cents: value.original_price_cents.map(|m| m.value()),
            discount_percent: value.discount_percent,
            is_free: value.is_free,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddWatchlistRequest {
    pub app_id: i64,
    pub target_price_cents: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWatchlistRequest {
    #[serde(default, deserialize_with = "deserialize_double_option")]
    pub target_price_cents: Option<Option<i64>>,
    pub is_active: Option<bool>,
}

fn deserialize_double_option<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

#[derive(Debug, Serialize)]
pub struct WatchlistItemDto {
    pub id: String,
    pub app_id: i64,
    pub title: String,
    pub header_image_url: String,
    pub target_price_cents: Option<i64>,
    pub currency: String,
    pub current_price_cents: Option<i64>,
    pub original_price_cents: Option<i64>,
    pub discount_percent: u8,
    pub is_free: bool,
    pub is_active: bool,
    pub is_target_met: bool,
    pub last_notified_price_cents: Option<i64>,
    pub last_notified_at: Option<DateTime<Utc>>,
    pub last_checked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<WatchlistItem> for WatchlistItemDto {
    fn from(value: WatchlistItem) -> Self {
        let is_target_met = value.is_target_met();
        Self {
            id: value.id,
            app_id: value.app_id.value(),
            title: value.title,
            header_image_url: value.header_image_url,
            target_price_cents: value.target_price_cents.map(|m| m.value()),
            currency: value.currency,
            current_price_cents: value.current_price_cents.map(|m| m.value()),
            original_price_cents: value.original_price_cents.map(|m| m.value()),
            discount_percent: value.discount_percent,
            is_free: value.is_free,
            is_active: value.is_active,
            is_target_met,
            last_notified_price_cents: value.last_notified_price_cents.map(|m| m.value()),
            last_notified_at: value.last_notified_at,
            last_checked_at: value.last_checked_at,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PriceHistoryPointDto {
    pub id: i64,
    pub watchlist_item_id: String,
    pub price_cents: i64,
    pub original_price_cents: i64,
    pub discount_percent: u8,
    pub recorded_at: DateTime<Utc>,
}

impl From<PriceHistoryPoint> for PriceHistoryPointDto {
    fn from(value: PriceHistoryPoint) -> Self {
        Self {
            id: value.id,
            watchlist_item_id: value.watchlist_item_id,
            price_cents: value.price_cents.value(),
            original_price_cents: value.original_price_cents.value(),
            discount_percent: value.discount_percent,
            recorded_at: value.recorded_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TriggerCheckResponse {
    pub status: &'static str,
}
