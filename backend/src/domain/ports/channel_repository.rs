use async_trait::async_trait;

use crate::domain::entities::NotificationChannel;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait ChannelRepository: Send + Sync {
    async fn list_all(&self) -> Result<Vec<NotificationChannel>, DomainError>;
    async fn list_enabled(&self) -> Result<Vec<NotificationChannel>, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<NotificationChannel>, DomainError>;
    async fn insert(&self, channel: &NotificationChannel) -> Result<(), DomainError>;
    async fn update(&self, channel: &NotificationChannel) -> Result<(), DomainError>;
    async fn delete(&self, id: &str) -> Result<bool, DomainError>;
}
