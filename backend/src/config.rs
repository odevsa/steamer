use std::env;
use std::path::PathBuf;

use crate::domain::entities::{DEFAULT_POLL_INTERVAL_SECS, MIN_POLL_INTERVAL_SECS};
use crate::domain::value_objects::CountryCode;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub data_dir: PathBuf,
    pub master_key: Option<String>,
    pub default_poll_interval_secs: u64,
    pub allow_http_webhooks: bool,
    pub default_country_code: CountryCode,
    pub default_ui_locale: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = env_first(&["HOST", "APP_HOST"]).unwrap_or_else(|| "0.0.0.0".to_string());
        let port = env_first(&["PORT", "APP_PORT"])
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8080);
        let data_dir = env::var("DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./data"));
        let master_key = env::var("STEAMER_MASTER_KEY")
            .ok()
            .filter(|v| !v.trim().is_empty());
        let default_poll_interval_secs = env::var("POLL_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(|v| v.max(MIN_POLL_INTERVAL_SECS))
            .unwrap_or(DEFAULT_POLL_INTERVAL_SECS);
        let allow_http_webhooks = env::var("ALLOW_HTTP_WEBHOOKS")
            .ok()
            .is_some_and(|v| v.eq_ignore_ascii_case("true") || v == "1");

        let default_country_code = detect_default_country_code();
        let default_ui_locale = detect_default_ui_locale();

        Self {
            host,
            port,
            data_dir,
            master_key,
            default_poll_interval_secs,
            allow_http_webhooks,
            default_country_code,
            default_ui_locale,
        }
    }

    pub fn database_path(&self) -> PathBuf {
        self.data_dir.join("steamer.db")
    }

    pub fn master_key_path(&self) -> PathBuf {
        self.data_dir.join("master.key")
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn env_first(keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| env::var(key).ok().filter(|v| !v.trim().is_empty()))
}

fn raw_os_locale() -> Option<String> {
    env_first(&["LC_ALL", "LC_MESSAGES", "LANG"])
}

fn detect_default_country_code() -> CountryCode {
    if let Ok(region) = env::var("STEAMER_DEFAULT_REGION") {
        if let Ok(code) = CountryCode::new(region) {
            return code;
        }
    }
    if let Some(locale) = raw_os_locale() {
        if let Some(code) = parse_country_from_locale(&locale) {
            return code;
        }
    }
    CountryCode::default_us()
}

fn detect_default_ui_locale() -> String {
    if let Some(locale) = raw_os_locale() {
        let lower = locale.to_ascii_lowercase();
        if lower.starts_with("pt") {
            return "pt-BR".to_string();
        }
        if lower.starts_with("en") {
            return "en".to_string();
        }
    }
    "en".to_string()
}

pub fn parse_country_from_locale(raw: &str) -> Option<CountryCode> {
    let base = raw.split('.').next()?.split('@').next()?.trim();
    if base.is_empty() || base.eq_ignore_ascii_case("C") || base.eq_ignore_ascii_case("POSIX") {
        return None;
    }
    let parts: Vec<&str> = base.split(['_', '-']).collect();
    if parts.len() >= 2 {
        return CountryCode::new(parts[1]).ok();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::parse_country_from_locale;

    #[test]
    fn parses_country_code_from_posix_locales() {
        assert_eq!(
            parse_country_from_locale("pt_BR.UTF-8").map(|c| c.to_string()),
            Some("BR".to_string())
        );
        assert_eq!(
            parse_country_from_locale("en-GB").map(|c| c.to_string()),
            Some("GB".to_string())
        );
        assert_eq!(parse_country_from_locale("C.UTF-8"), None);
        assert_eq!(parse_country_from_locale("POSIX"), None);
    }
}
