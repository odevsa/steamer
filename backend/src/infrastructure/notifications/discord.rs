use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::domain::entities::{ChannelConfig, ChannelType, PriceAlertPayload};
use crate::domain::errors::DomainError;
use crate::domain::ports::{Notifier, UrlValidator};
use crate::infrastructure::notifications::formatting::format_money;

pub struct DiscordNotifier {
    client: Client,
    url_validator: Arc<dyn UrlValidator>,
}

impl DiscordNotifier {
    pub fn new(url_validator: Arc<dyn UrlValidator>) -> Result<Self, DomainError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| DomainError::Internal(format!("failed to build HTTP client: {e}")))?;
        Ok(Self {
            client,
            url_validator,
        })
    }
}

#[async_trait]
impl Notifier for DiscordNotifier {
    fn channel_type(&self) -> ChannelType {
        ChannelType::Discord
    }

    async fn send(
        &self,
        config: &ChannelConfig,
        alert: &PriceAlertPayload,
    ) -> Result<(), DomainError> {
        let ChannelConfig::Discord { webhook_url } = config else {
            return Err(DomainError::Validation(
                "invalid config for discord notifier".to_string(),
            ));
        };

        let target_url = webhook_url.expose().trim();
        self.url_validator.validate_outbound_url(target_url).await?;

        let current_formatted = format_money(alert.current_price_cents, &alert.currency);
        let original_formatted = format_money(alert.original_price_cents, &alert.currency);
        let target_line = match alert.target_price_cents {
            Some(target) => format_money(target, &alert.currency),
            None => "Any discount".to_string(),
        };

        let payload = json!({
            "embeds": [{
                "title": format!("{} (-{}%)", alert.title, alert.discount_percent),
                "url": alert.store_url,
                "color": 2278750,
                "thumbnail": { "url": alert.header_image_url },
                "fields": [
                    { "name": "Current Price", "value": current_formatted, "inline": true },
                    { "name": "Original Price", "value": original_formatted, "inline": true },
                    { "name": "Target", "value": target_line, "inline": true }
                ],
                "timestamp": alert.triggered_at.to_rfc3339()
            }]
        });

        let response = self
            .client
            .post(target_url)
            .json(&payload)
            .send()
            .await
            .map_err(|_| {
                DomainError::ExternalService("discord webhook request failed".to_string())
            })?;

        if !response.status().is_success() {
            return Err(DomainError::ExternalService(format!(
                "discord webhook returned HTTP {}",
                response.status().as_u16()
            )));
        }

        Ok(())
    }
}
