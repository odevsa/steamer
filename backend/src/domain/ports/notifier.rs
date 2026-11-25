use async_trait::async_trait;

use crate::domain::entities::{ChannelConfig, ChannelType, PriceAlertPayload};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait Notifier: Send + Sync {
    fn channel_type(&self) -> ChannelType;
    async fn send(
        &self,
        config: &ChannelConfig,
        alert: &PriceAlertPayload,
    ) -> Result<(), DomainError>;
}
