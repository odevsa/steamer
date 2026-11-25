use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::domain::entities::{ChannelConfig, NotificationChannel, PriceAlertPayload};
use crate::domain::errors::DomainError;
use crate::domain::ports::{AlertDispatcher, ChannelRepository, UrlValidator};
use crate::domain::value_objects::{AppId, MoneyCents};

#[derive(Clone)]
pub struct ChannelsUseCase {
    channel_repo: Arc<dyn ChannelRepository>,
    url_validator: Arc<dyn UrlValidator>,
    dispatcher: Arc<dyn AlertDispatcher>,
}

impl ChannelsUseCase {
    pub fn new(
        channel_repo: Arc<dyn ChannelRepository>,
        url_validator: Arc<dyn UrlValidator>,
        dispatcher: Arc<dyn AlertDispatcher>,
    ) -> Self {
        Self {
            channel_repo,
            url_validator,
            dispatcher,
        }
    }

    pub async fn list_all(&self) -> Result<Vec<NotificationChannel>, DomainError> {
        self.channel_repo.list_all().await
    }

    pub async fn create_channel(
        &self,
        name: String,
        config: ChannelConfig,
        is_enabled: bool,
    ) -> Result<NotificationChannel, DomainError> {
        let trimmed_name = validate_channel_name(&name)?;
        self.validate_config(&config).await?;

        let now = Utc::now();
        let channel = NotificationChannel {
            id: Uuid::new_v4().to_string(),
            name: trimmed_name,
            channel_type: config.channel_type(),
            config,
            is_enabled,
            created_at: now,
            updated_at: now,
        };

        self.channel_repo.insert(&channel).await?;
        Ok(channel)
    }

    pub async fn update_channel(
        &self,
        id: &str,
        name: Option<String>,
        config: Option<ChannelConfig>,
        is_enabled: Option<bool>,
    ) -> Result<NotificationChannel, DomainError> {
        let mut channel =
            self.channel_repo.find_by_id(id).await?.ok_or_else(|| {
                DomainError::NotFound("notification channel not found".to_string())
            })?;

        if let Some(new_name) = name {
            channel.name = validate_channel_name(&new_name)?;
        }

        if let Some(new_config) = config {
            self.validate_config(&new_config).await?;
            channel.channel_type = new_config.channel_type();
            channel.config = new_config;
        }

        if let Some(enabled) = is_enabled {
            channel.is_enabled = enabled;
        }

        channel.updated_at = Utc::now();
        self.channel_repo.update(&channel).await?;
        Ok(channel)
    }

    pub async fn delete_channel(&self, id: &str) -> Result<(), DomainError> {
        if !self.channel_repo.delete(id).await? {
            return Err(DomainError::NotFound(
                "notification channel not found".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn test_channel(&self, id: &str) -> Result<(), DomainError> {
        let channel =
            self.channel_repo.find_by_id(id).await?.ok_or_else(|| {
                DomainError::NotFound("notification channel not found".to_string())
            })?;

        let sample_payload = PriceAlertPayload {
            event: "price_alert_test".to_string(),
            app_id: AppId::new(1091500)?,
            title: "Steamer Test Alert".to_string(),
            currency: "USD".to_string(),
            current_price_cents: MoneyCents::new(1999)?,
            original_price_cents: MoneyCents::new(5999)?,
            discount_percent: 67,
            target_price_cents: Some(MoneyCents::new(2500)?),
            store_url: "https://store.steampowered.com/app/1091500".to_string(),
            header_image_url:
                "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1091500/header.jpg"
                    .to_string(),
            triggered_at: Utc::now(),
        };

        self.dispatcher
            .dispatch(&channel.config, &sample_payload)
            .await
    }

    async fn validate_config(&self, config: &ChannelConfig) -> Result<(), DomainError> {
        match config {
            ChannelConfig::Discord { webhook_url } => {
                self.url_validator
                    .validate_outbound_url(webhook_url.expose())
                    .await
            }
            ChannelConfig::Telegram { bot_token, chat_id } => {
                if bot_token.is_empty() || chat_id.is_empty() {
                    return Err(DomainError::Validation(
                        "telegram bot_token and chat_id are required".to_string(),
                    ));
                }
                Ok(())
            }
            ChannelConfig::Ntfy {
                server_url, topic, ..
            } => {
                let trimmed_topic = topic.trim().trim_matches('/');
                if trimmed_topic.is_empty() || trimmed_topic.contains('/') {
                    return Err(DomainError::Validation(
                        "ntfy topic must be a single non-empty segment".to_string(),
                    ));
                }
                let url = format!("{}/{}", server_url.trim_end_matches('/'), trimmed_topic);
                self.url_validator.validate_outbound_url(&url).await
            }
            ChannelConfig::Webhook { url, .. } => {
                self.url_validator.validate_outbound_url(url.expose()).await
            }
        }
    }
}

fn validate_channel_name(raw: &str) -> Result<String, DomainError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() > 80 {
        return Err(DomainError::Validation(
            "channel name must be between 1 and 80 characters".to_string(),
        ));
    }
    Ok(trimmed.to_string())
}
