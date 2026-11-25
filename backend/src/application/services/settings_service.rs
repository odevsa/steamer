use std::sync::Arc;

use crate::config::AppConfig;
use crate::domain::entities::AppSettings;
use crate::domain::errors::DomainError;
use crate::domain::ports::SettingsRepository;
use crate::domain::value_objects::CountryCode;

#[derive(Clone)]
pub struct SettingsService {
    repo: Arc<dyn SettingsRepository>,
    default_country_code: CountryCode,
    default_ui_locale: String,
    default_poll_interval_secs: u64,
}

impl SettingsService {
    pub fn new(repo: Arc<dyn SettingsRepository>, config: &AppConfig) -> Self {
        Self {
            repo,
            default_country_code: config.default_country_code.clone(),
            default_ui_locale: config.default_ui_locale.clone(),
            default_poll_interval_secs: config.default_poll_interval_secs,
        }
    }

    pub fn detected_defaults(&self) -> AppSettings {
        AppSettings {
            steam_country_code: self.default_country_code.clone(),
            ui_locale: self.default_ui_locale.clone(),
            poll_interval_secs: self.default_poll_interval_secs,
        }
    }

    pub async fn get_or_init(&self) -> Result<AppSettings, DomainError> {
        if let Some(existing) = self.repo.get().await? {
            return Ok(existing);
        }
        let defaults = self.detected_defaults();
        self.repo.upsert(&defaults).await?;
        Ok(defaults)
    }

    pub async fn update(&self, settings: &AppSettings) -> Result<AppSettings, DomainError> {
        self.repo.upsert(settings).await?;
        Ok(settings.clone())
    }
}
