use crate::application::services::SettingsService;
use crate::domain::entities::AppSettings;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::CountryCode;

#[derive(Clone)]
pub struct SettingsUseCase {
    settings_service: SettingsService,
}

pub struct SettingsView {
    pub current: AppSettings,
    pub detected_defaults: AppSettings,
}

impl SettingsUseCase {
    pub fn new(settings_service: SettingsService) -> Self {
        Self { settings_service }
    }

    pub async fn get(&self) -> Result<SettingsView, DomainError> {
        let current = self.settings_service.get_or_init().await?;
        let detected_defaults = self.settings_service.detected_defaults();
        Ok(SettingsView {
            current,
            detected_defaults,
        })
    }

    pub async fn update(
        &self,
        steam_country_code: Option<String>,
        ui_locale: Option<String>,
        poll_interval_secs: Option<u64>,
    ) -> Result<SettingsView, DomainError> {
        let current = self.settings_service.get_or_init().await?;
        let next_country = match steam_country_code {
            Some(code) => CountryCode::new(code)?,
            None => current.steam_country_code,
        };
        let next_locale = ui_locale.unwrap_or(current.ui_locale);
        let next_interval = poll_interval_secs.unwrap_or(current.poll_interval_secs);

        let updated = AppSettings::new(next_country, next_locale, next_interval)?;
        let saved = self.settings_service.update(&updated).await?;
        let detected_defaults = self.settings_service.detected_defaults();
        Ok(SettingsView {
            current: saved,
            detected_defaults,
        })
    }
}
