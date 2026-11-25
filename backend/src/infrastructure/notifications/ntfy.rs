use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::domain::entities::{ChannelConfig, ChannelType, PriceAlertPayload};
use crate::domain::errors::DomainError;
use crate::domain::ports::{Notifier, UrlValidator};
use crate::infrastructure::notifications::formatting::format_money;

pub struct NtfyNotifier {
    client: Client,
    url_validator: Arc<dyn UrlValidator>,
}

impl NtfyNotifier {
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
impl Notifier for NtfyNotifier {
    fn channel_type(&self) -> ChannelType {
        ChannelType::Ntfy
    }

    async fn send(
        &self,
        config: &ChannelConfig,
        alert: &PriceAlertPayload,
    ) -> Result<(), DomainError> {
        let ChannelConfig::Ntfy {
            server_url,
            topic,
            access_token,
        } = config
        else {
            return Err(DomainError::Validation(
                "invalid config for ntfy notifier".to_string(),
            ));
        };

        let trimmed_topic = topic.trim().trim_matches('/');
        if trimmed_topic.is_empty() || trimmed_topic.contains('/') {
            return Err(DomainError::Validation(
                "ntfy topic must be a single non-empty segment".to_string(),
            ));
        }

        let target_url = format!("{}/{}", server_url.trim_end_matches('/'), trimmed_topic);
        self.url_validator
            .validate_outbound_url(&target_url)
            .await?;

        let current_formatted = format_money(alert.current_price_cents, &alert.currency);
        let original_formatted = format_money(alert.original_price_cents, &alert.currency);
        let body = format!(
            "Price dropped to {current_formatted} (was {original_formatted}, -{}%)",
            alert.discount_percent
        );

        let mut request = self
            .client
            .post(&target_url)
            .header(
                "Title",
                format!("{} (-{}%)", alert.title, alert.discount_percent),
            )
            .header("Priority", "default")
            .header("Tags", "moneybag")
            .header("Click", &alert.store_url)
            .body(body);

        if let Some(token) = access_token {
            if !token.is_empty() {
                request = request.bearer_auth(token.expose());
            }
        }

        let response = request
            .send()
            .await
            .map_err(|_| DomainError::ExternalService("ntfy request failed".to_string()))?;

        if !response.status().is_success() {
            return Err(DomainError::ExternalService(format!(
                "ntfy returned HTTP {}",
                response.status().as_u16()
            )));
        }

        Ok(())
    }
}
