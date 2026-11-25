use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entities::{AlertLog, AlertStatus};

#[derive(Debug, Deserialize)]
pub struct AlertLogsQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    50
}

#[derive(Debug, Serialize)]
pub struct AlertLogDto {
    pub id: i64,
    pub watchlist_item_id: String,
    pub channel_id: Option<String>,
    pub channel_name: String,
    pub triggered_price_cents: i64,
    pub discount_percent: u8,
    pub status: AlertStatus,
    pub error_summary: Option<String>,
    pub sent_at: DateTime<Utc>,
}

impl From<AlertLog> for AlertLogDto {
    fn from(value: AlertLog) -> Self {
        Self {
            id: value.id,
            watchlist_item_id: value.watchlist_item_id,
            channel_id: value.channel_id,
            channel_name: value.channel_name,
            triggered_price_cents: value.triggered_price_cents.value(),
            discount_percent: value.discount_percent,
            status: value.status,
            error_summary: value.error_summary,
            sent_at: value.sent_at,
        }
    }
}
