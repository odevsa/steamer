use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};

use crate::domain::entities::{AlertLog, AlertStatus, NewAlertLog};
use crate::domain::errors::DomainError;
use crate::domain::ports::AlertLogRepository;
use crate::domain::value_objects::MoneyCents;

#[derive(FromRow)]
struct AlertLogRow {
    id: i64,
    watchlist_item_id: String,
    channel_id: Option<String>,
    channel_name: String,
    triggered_price_cents: i64,
    discount_percent: i64,
    status: String,
    error_summary: Option<String>,
    sent_at: String,
}

impl TryFrom<AlertLogRow> for AlertLog {
    type Error = DomainError;

    fn try_from(row: AlertLogRow) -> Result<Self, Self::Error> {
        let status = match row.status.as_str() {
            "sent" => AlertStatus::Sent,
            "failed" => AlertStatus::Failed,
            _ => return Err(DomainError::Storage("invalid alert status".to_string())),
        };
        Ok(Self {
            id: row.id,
            watchlist_item_id: row.watchlist_item_id,
            channel_id: row.channel_id,
            channel_name: row.channel_name,
            triggered_price_cents: MoneyCents::new(row.triggered_price_cents)?,
            discount_percent: u8::try_from(row.discount_percent).unwrap_or(0).min(100),
            status,
            error_summary: row.error_summary,
            sent_at: parse_rfc3339(&row.sent_at)?,
        })
    }
}

pub struct SqliteAlertLogRepository {
    pool: SqlitePool,
}

impl SqliteAlertLogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AlertLogRepository for SqliteAlertLogRepository {
    async fn insert(&self, entry: &NewAlertLog) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO alert_logs (watchlist_item_id, channel_id, channel_name, triggered_price_cents, discount_percent, status, error_summary, sent_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind(&entry.watchlist_item_id)
        .bind(&entry.channel_id)
        .bind(&entry.channel_name)
        .bind(entry.triggered_price_cents.value())
        .bind(i64::from(entry.discount_percent))
        .bind(entry.status.as_str())
        .bind(&entry.error_summary)
        .bind(entry.sent_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to insert alert log: {e}")))?;
        Ok(())
    }

    async fn list_paginated(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AlertLog>, DomainError> {
        let bounded_limit = i64::try_from(limit.clamp(1, 200)).unwrap_or(50);
        let bounded_offset = i64::try_from(offset).unwrap_or(0);
        let rows: Vec<AlertLogRow> = sqlx::query_as(
            "SELECT id, watchlist_item_id, channel_id, channel_name, triggered_price_cents, discount_percent, status, error_summary, sent_at FROM alert_logs ORDER BY sent_at DESC, id DESC LIMIT ?1 OFFSET ?2",
        )
        .bind(bounded_limit)
        .bind(bounded_offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to query alert logs: {e}")))?;
        rows.into_iter().map(AlertLog::try_from).collect()
    }
}

fn parse_rfc3339(raw: &str) -> Result<DateTime<Utc>, DomainError> {
    DateTime::parse_from_rfc3339(raw)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| DomainError::Storage("invalid timestamp stored in database".to_string()))
}
