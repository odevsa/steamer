use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn expose(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }

    pub fn masked(&self) -> String {
        let chars: Vec<char> = self.0.chars().collect();
        if chars.len() <= 4 {
            return "••••".to_string();
        }
        let tail: String = chars[chars.len() - 4..].iter().collect();
        format!("••••••••{tail}")
    }

    pub fn masked_url(&self) -> String {
        let trimmed = self.0.trim();
        if let Some(scheme_end) = trimmed.find("://") {
            let after_scheme = &trimmed[scheme_end + 3..];
            if let Some(slash_idx) = after_scheme.find('/') {
                let origin = &trimmed[..scheme_end + 3 + slash_idx];
                let chars: Vec<char> = trimmed.chars().collect();
                let tail: String = chars[chars.len().saturating_sub(4)..].iter().collect();
                return format!("{origin}/••••{tail}");
            }
        }
        self.masked()
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SecretString([REDACTED])")
    }
}

impl std::fmt::Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

#[cfg(test)]
mod tests {
    use super::SecretString;

    #[test]
    fn redacts_debug_and_display() {
        let secret = SecretString::new("super-secret-token-9876");
        assert_eq!(format!("{secret}"), "[REDACTED]");
        assert_eq!(format!("{secret:?}"), "SecretString([REDACTED])");
        assert_eq!(secret.masked(), "••••••••9876");
    }

    #[test]
    fn masks_url_preserving_origin_and_tail() {
        let secret = SecretString::new("https://discord.com/api/webhooks/123456/abcdef1234");
        assert_eq!(secret.masked_url(), "https://discord.com/••••1234");
    }
}
