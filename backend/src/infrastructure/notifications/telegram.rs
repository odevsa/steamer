use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::domain::entities::{ChannelConfig, ChannelType, PriceAlertPayload};
use crate::domain::errors::DomainError;
use crate::domain::ports::Notifier;
use crate::infrastructure::notifications::formatting::{escape_html, format_money};

pub struct TelegramNotifier {
    client: Client,
}

impl TelegramNotifier {
    pub fn new() -> Result<Self, DomainError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| DomainError::Internal(format!("failed to build HTTP client: {e}")))?;
        Ok(Self { client })
    }
}

#[async_trait]
impl Notifier for TelegramNotifier {
    fn channel_type(&self) -> ChannelType {
        ChannelType::Telegram
    }

    async fn send(
        &self,
        config: &ChannelConfig,
        alert: &PriceAlertPayload,
    ) -> Result<(), DomainError> {
        let ChannelConfig::Telegram { bot_token, chat_id } = config else {
            return Err(DomainError::Validation(
                "invalid config for telegram notifier".to_string(),
            ));
        };

        let token = bot_token.expose().trim();
        let chat = chat_id.expose().trim();
        if token.is_empty() || chat.is_empty() {
            return Err(DomainError::Validation(
                "telegram bot_token and chat_id are required".to_string(),
            ));
        }

        let current_formatted = format_money(alert.current_price_cents, &alert.currency);
        let original_formatted = format_money(alert.original_price_cents, &alert.currency);
        let safe_title = escape_html(&alert.title);
        let safe_url = escape_html(&alert.store_url);

        let text = format!(
            "<b>{safe_title}</b> (-{discount}%)\nPrice: <b>{current_formatted}</b> (was <s>{original_formatted}</s>)\n<a href=\"{safe_url}\">Open on Steam</a>",
            discount = alert.discount_percent,
        );

        let endpoint = format!("https://api.telegram.org/bot{token}/sendMessage");
        let payload = json!({
            "chat_id": chat,
            "text": text,
            "parse_mode": "HTML",
            "disable_web_page_preview": false
        });

        let response = self
            .client
            .post(endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|_| DomainError::ExternalService("telegram API request failed".to_string()))?;

        if !response.status().is_success() {
            return Err(DomainError::ExternalService(format!(
                "telegram API returned HTTP {}",
                response.status().as_u16()
            )));
        }

        Ok(())
    }
}
