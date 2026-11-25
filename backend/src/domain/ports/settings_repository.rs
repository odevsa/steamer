use async_trait::async_trait;

use crate::domain::entities::AppSettings;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn get(&self) -> Result<Option<AppSettings>, DomainError>;
    async fn upsert(&self, settings: &AppSettings) -> Result<(), DomainError>;
}
