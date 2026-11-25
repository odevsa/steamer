use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Client;

use crate::domain::entities::{ChannelConfig, ChannelType, PriceAlertPayload};
use crate::domain::errors::DomainError;
use crate::domain::ports::{Notifier, UrlValidator};

pub struct WebhookNotifier {
    client: Client,
    url_validator: Arc<dyn UrlValidator>,
}

impl WebhookNotifier {
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
impl Notifier for WebhookNotifier {
    fn channel_type(&self) -> ChannelType {
        ChannelType::Webhook
    }

    async fn send(
        &self,
        config: &ChannelConfig,
        alert: &PriceAlertPayload,
    ) -> Result<(), DomainError> {
        let ChannelConfig::Webhook {
            url,
            secret_header_name,
            secret_header_value,
        } = config
        else {
            return Err(DomainError::Validation(
                "invalid config for webhook notifier".to_string(),
            ));
        };

        let target_url = url.expose().trim();
        self.url_validator.validate_outbound_url(target_url).await?;

        let mut request = self.client.post(target_url).json(alert);

        if let (Some(name), Some(value)) = (secret_header_name, secret_header_value) {
            let trimmed_name = name.trim();
            if !trimmed_name.is_empty() && !value.is_empty() {
                let header_name =
                    HeaderName::from_bytes(trimmed_name.as_bytes()).map_err(|_| {
                        DomainError::Validation("invalid custom webhook header name".to_string())
                    })?;
                let header_value = HeaderValue::from_str(value.expose()).map_err(|_| {
                    DomainError::Validation("invalid custom webhook header value".to_string())
                })?;
                request = request.header(header_name, header_value);
            }
        }

        let response = request
            .send()
            .await
            .map_err(|_| DomainError::ExternalService("webhook request failed".to_string()))?;

        if !response.status().is_success() {
            return Err(DomainError::ExternalService(format!(
                "webhook returned HTTP {}",
                response.status().as_u16()
            )));
        }

        Ok(())
    }
}
