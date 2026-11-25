use serde::{Deserialize, Serialize};

use crate::application::use_cases::SettingsView;
use crate::domain::entities::AppSettings;

#[derive(Debug, Serialize)]
pub struct SettingsFieldsDto {
    pub steam_country_code: String,
    pub ui_locale: String,
    pub poll_interval_secs: u64,
}

impl From<AppSettings> for SettingsFieldsDto {
    fn from(value: AppSettings) -> Self {
        Self {
            steam_country_code: value.steam_country_code.to_string(),
            ui_locale: value.ui_locale,
            poll_interval_secs: value.poll_interval_secs,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SettingsResponse {
    pub steam_country_code: String,
    pub ui_locale: String,
    pub poll_interval_secs: u64,
    pub detected_defaults: SettingsFieldsDto,
}

impl From<SettingsView> for SettingsResponse {
    fn from(view: SettingsView) -> Self {
        Self {
            steam_country_code: view.current.steam_country_code.to_string(),
            ui_locale: view.current.ui_locale,
            poll_interval_secs: view.current.poll_interval_secs,
            detected_defaults: view.detected_defaults.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub steam_country_code: Option<String>,
    pub ui_locale: Option<String>,
    pub poll_interval_secs: Option<u64>,
}
