use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};

use crate::domain::entities::AppSettings;
use crate::domain::errors::DomainError;
use crate::domain::ports::SettingsRepository;
use crate::domain::value_objects::CountryCode;

#[derive(FromRow)]
struct SettingsRow {
    steam_country_code: String,
    ui_locale: String,
    poll_interval_secs: i64,
}

pub struct SqliteSettingsRepository {
    pool: SqlitePool,
}

impl SqliteSettingsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
    async fn get(&self) -> Result<Option<AppSettings>, DomainError> {
        let row: Option<SettingsRow> = sqlx::query_as(
            "SELECT steam_country_code, ui_locale, poll_interval_secs FROM app_settings WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to query settings: {e}")))?;

        row.map(|r| {
            let country = CountryCode::new(r.steam_country_code)?;
            let interval = u64::try_from(r.poll_interval_secs).unwrap_or(3600);
            AppSettings::new(country, r.ui_locale, interval)
        })
        .transpose()
    }

    async fn upsert(&self, settings: &AppSettings) -> Result<(), DomainError> {
        let interval = i64::try_from(settings.poll_interval_secs)
            .map_err(|_| DomainError::Validation("poll interval out of range".to_string()))?;
        sqlx::query(
            "INSERT INTO app_settings (id, steam_country_code, ui_locale, poll_interval_secs) VALUES (1, ?1, ?2, ?3) ON CONFLICT(id) DO UPDATE SET steam_country_code = excluded.steam_country_code, ui_locale = excluded.ui_locale, poll_interval_secs = excluded.poll_interval_secs",
        )
        .bind(settings.steam_country_code.as_str())
        .bind(&settings.ui_locale)
        .bind(interval)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to save settings: {e}")))?;
        Ok(())
    }
}
