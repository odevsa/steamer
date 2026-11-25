use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::entities::{ChannelConfig, ChannelType, PriceAlertPayload};
use crate::domain::errors::DomainError;
use crate::domain::ports::{AlertDispatcher, Notifier, UrlValidator};
use crate::infrastructure::notifications::discord::DiscordNotifier;
use crate::infrastructure::notifications::ntfy::NtfyNotifier;
use crate::infrastructure::notifications::telegram::TelegramNotifier;
use crate::infrastructure::notifications::webhook::WebhookNotifier;

pub struct NotifierRegistry {
    notifiers: HashMap<ChannelType, Arc<dyn Notifier>>,
}

impl NotifierRegistry {
    pub fn with_defaults(url_validator: Arc<dyn UrlValidator>) -> Result<Self, DomainError> {
        let list: Vec<Arc<dyn Notifier>> = vec![
            Arc::new(DiscordNotifier::new(url_validator.clone())?),
            Arc::new(TelegramNotifier::new()?),
            Arc::new(NtfyNotifier::new(url_validator.clone())?),
            Arc::new(WebhookNotifier::new(url_validator)?),
        ];

        let mut map = HashMap::new();
        for notifier in list {
            map.insert(notifier.channel_type(), notifier);
        }
        Ok(Self { notifiers: map })
    }
}

#[async_trait]
impl AlertDispatcher for NotifierRegistry {
    async fn dispatch(
        &self,
        config: &ChannelConfig,
        alert: &PriceAlertPayload,
    ) -> Result<(), DomainError> {
        let notifier = self.notifiers.get(&config.channel_type()).ok_or_else(|| {
            DomainError::Internal("no notifier registered for channel type".to_string())
        })?;
        notifier.send(config, alert).await
    }
}
