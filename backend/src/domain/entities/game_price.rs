use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{AppId, MoneyCents};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSearchResult {
    pub app_id: AppId,
    pub title: String,
    pub header_image_url: String,
    pub currency: Option<String>,
    pub current_price_cents: Option<MoneyCents>,
    pub original_price_cents: Option<MoneyCents>,
    pub discount_percent: u8,
    pub is_free: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GamePriceSnapshot {
    pub app_id: AppId,
    pub title: String,
    pub header_image_url: String,
    pub currency: String,
    pub current_price_cents: MoneyCents,
    pub original_price_cents: MoneyCents,
    pub discount_percent: u8,
    pub is_free: bool,
    pub fetched_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriceHistoryPoint {
    pub id: i64,
    pub watchlist_item_id: String,
    pub price_cents: MoneyCents,
    pub original_price_cents: MoneyCents,
    pub discount_percent: u8,
    pub recorded_at: DateTime<Utc>,
}
