use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::MoneyCents;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    Sent,
    Failed,
}

impl AlertStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sent => "sent",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlertLog {
    pub id: i64,
    pub watchlist_item_id: String,
    pub channel_id: Option<String>,
    pub channel_name: String,
    pub triggered_price_cents: MoneyCents,
    pub discount_percent: u8,
    pub status: AlertStatus,
    pub error_summary: Option<String>,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewAlertLog {
    pub watchlist_item_id: String,
    pub channel_id: Option<String>,
    pub channel_name: String,
    pub triggered_price_cents: MoneyCents,
    pub discount_percent: u8,
    pub status: AlertStatus,
    pub error_summary: Option<String>,
    pub sent_at: DateTime<Utc>,
}
