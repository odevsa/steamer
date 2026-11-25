use crate::domain::errors::DomainError;

pub trait SecretCipher: Send + Sync {
    fn encrypt(&self, plaintext: &str) -> Result<String, DomainError>;
    fn decrypt(&self, ciphertext: &str) -> Result<String, DomainError>;
}

#[async_trait::async_trait]
pub trait UrlValidator: Send + Sync {
    async fn validate_outbound_url(&self, url: &str) -> Result<(), DomainError>;
}
