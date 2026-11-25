use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::errors::DomainError;
use crate::domain::value_objects::{AppId, MoneyCents, SecretString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    Discord,
    Telegram,
    Ntfy,
    Webhook,
}

impl ChannelType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discord => "discord",
            Self::Telegram => "telegram",
            Self::Ntfy => "ntfy",
            Self::Webhook => "webhook",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DomainError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "discord" => Ok(Self::Discord),
            "telegram" => Ok(Self::Telegram),
            "ntfy" => Ok(Self::Ntfy),
            "webhook" => Ok(Self::Webhook),
            _ => Err(DomainError::Validation(
                "unsupported notification channel type".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChannelConfig {
    Discord {
        webhook_url: SecretString,
    },
    Telegram {
        bot_token: SecretString,
        chat_id: SecretString,
    },
    Ntfy {
        server_url: String,
        topic: String,
        access_token: Option<SecretString>,
    },
    Webhook {
        url: SecretString,
        secret_header_name: Option<String>,
        secret_header_value: Option<SecretString>,
    },
}

impl ChannelConfig {
    pub fn channel_type(&self) -> ChannelType {
        match self {
            Self::Discord { .. } => ChannelType::Discord,
            Self::Telegram { .. } => ChannelType::Telegram,
            Self::Ntfy { .. } => ChannelType::Ntfy,
            Self::Webhook { .. } => ChannelType::Webhook,
        }
    }

    pub fn masked(&self) -> MaskedChannelConfig {
        match self {
            Self::Discord { webhook_url } => MaskedChannelConfig::Discord {
                webhook_url: webhook_url.masked_url(),
            },
            Self::Telegram { bot_token, chat_id } => MaskedChannelConfig::Telegram {
                bot_token: bot_token.masked(),
                chat_id: chat_id.masked(),
            },
            Self::Ntfy {
                server_url,
                topic,
                access_token,
            } => MaskedChannelConfig::Ntfy {
                server_url: server_url.clone(),
                topic: topic.clone(),
                has_access_token: access_token.as_ref().is_some_and(|t| !t.is_empty()),
            },
            Self::Webhook {
                url,
                secret_header_name,
                secret_header_value,
            } => MaskedChannelConfig::Webhook {
                url: url.masked_url(),
                secret_header_name: secret_header_name.clone(),
                has_secret_header_value: secret_header_value
                    .as_ref()
                    .is_some_and(|v| !v.is_empty()),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MaskedChannelConfig {
    Discord {
        webhook_url: String,
    },
    Telegram {
        bot_token: String,
        chat_id: String,
    },
    Ntfy {
        server_url: String,
        topic: String,
        has_access_token: bool,
    },
    Webhook {
        url: String,
        secret_header_name: Option<String>,
        has_secret_header_value: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationChannel {
    pub id: String,
    pub name: String,
    pub channel_type: ChannelType,
    pub config: ChannelConfig,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriceAlertPayload {
    pub event: String,
    pub app_id: AppId,
    pub title: String,
    pub currency: String,
    pub current_price_cents: MoneyCents,
    pub original_price_cents: MoneyCents,
    pub discount_percent: u8,
    pub target_price_cents: Option<MoneyCents>,
    pub store_url: String,
    pub header_image_url: String,
    pub triggered_at: DateTime<Utc>,
}
