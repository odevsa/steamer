use serde::{Deserialize, Serialize};

use crate::domain::errors::DomainError;
use crate::domain::value_objects::CountryCode;

pub const MIN_POLL_INTERVAL_SECS: u64 = 300;
pub const DEFAULT_POLL_INTERVAL_SECS: u64 = 3600;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppSettings {
    pub steam_country_code: CountryCode,
    pub ui_locale: String,
    pub poll_interval_secs: u64,
}

impl AppSettings {
    pub fn new(
        steam_country_code: CountryCode,
        ui_locale: impl Into<String>,
        poll_interval_secs: u64,
    ) -> Result<Self, DomainError> {
        if poll_interval_secs < MIN_POLL_INTERVAL_SECS {
            return Err(DomainError::Validation(format!(
                "poll_interval_secs must be at least {MIN_POLL_INTERVAL_SECS}"
            )));
        }
        let locale = normalize_ui_locale(&ui_locale.into())?;
        Ok(Self {
            steam_country_code,
            ui_locale: locale,
            poll_interval_secs,
        })
    }
}

pub fn normalize_ui_locale(raw: &str) -> Result<String, DomainError> {
    let trimmed = raw.trim();
    if trimmed.eq_ignore_ascii_case("auto") {
        return Ok("auto".to_string());
    }
    if trimmed.eq_ignore_ascii_case("en") || trimmed.to_ascii_lowercase().starts_with("en-") {
        return Ok("en".to_string());
    }
    if trimmed.eq_ignore_ascii_case("pt-br") || trimmed.eq_ignore_ascii_case("pt") {
        return Ok("pt-BR".to_string());
    }
    Err(DomainError::Validation(
        "ui_locale must be 'auto', 'en', or 'pt-BR'".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{normalize_ui_locale, AppSettings, MIN_POLL_INTERVAL_SECS};
    use crate::domain::value_objects::CountryCode;

    #[test]
    fn rejects_poll_interval_below_minimum() {
        let result = AppSettings::new(CountryCode::default_us(), "en", MIN_POLL_INTERVAL_SECS - 1);
        assert!(result.is_err());
    }

    #[test]
    fn normalizes_supported_locales() {
        assert!(normalize_ui_locale("pt_br").is_err());
        assert_eq!(normalize_ui_locale("pt-br").expect("valid"), "pt-BR");
        assert_eq!(normalize_ui_locale("en-US").expect("valid"), "en");
        assert_eq!(normalize_ui_locale("AUTO").expect("valid"), "auto");
    }
}
