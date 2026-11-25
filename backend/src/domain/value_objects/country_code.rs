use serde::{Deserialize, Serialize};

use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode(String);

impl CountryCode {
    pub fn new(code: impl Into<String>) -> Result<Self, DomainError> {
        let normalized = code.into().trim().to_ascii_uppercase();
        if normalized.len() != 2 || !normalized.bytes().all(|b| b.is_ascii_uppercase()) {
            return Err(DomainError::Validation(
                "country_code must be a 2-letter ISO alpha-2 code".to_string(),
            ));
        }
        Ok(Self(normalized))
    }

    pub fn default_us() -> Self {
        Self("US".to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::CountryCode;

    #[test]
    fn normalizes_valid_country_code() {
        let code = CountryCode::new(" br ").expect("valid country code");
        assert_eq!(code.as_str(), "BR");
    }

    #[test]
    fn rejects_invalid_country_codes() {
        assert!(CountryCode::new("USA").is_err());
        assert!(CountryCode::new("1A").is_err());
        assert!(CountryCode::new("").is_err());
    }
}
