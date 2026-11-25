use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entities::{
    ChannelConfig, ChannelType, MaskedChannelConfig, NotificationChannel,
};
use crate::domain::value_objects::SecretString;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChannelConfigInput {
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
        access_token: Option<String>,
    },
    Webhook {
        url: String,
        secret_header_name: Option<String>,
        secret_header_value: Option<String>,
    },
}

impl From<ChannelConfigInput> for ChannelConfig {
    fn from(input: ChannelConfigInput) -> Self {
        match input {
            ChannelConfigInput::Discord { webhook_url } => Self::Discord {
                webhook_url: SecretString::new(webhook_url),
            },
            ChannelConfigInput::Telegram { bot_token, chat_id } => Self::Telegram {
                bot_token: SecretString::new(bot_token),
                chat_id: SecretString::new(chat_id),
            },
            ChannelConfigInput::Ntfy {
                server_url,
                topic,
                access_token,
            } => Self::Ntfy {
                server_url,
                topic,
                access_token: access_token
                    .filter(|v| !v.trim().is_empty())
                    .map(SecretString::new),
            },
            ChannelConfigInput::Webhook {
                url,
                secret_header_name,
                secret_header_value,
            } => Self::Webhook {
                url: SecretString::new(url),
                secret_header_name: secret_header_name.filter(|v| !v.trim().is_empty()),
                secret_header_value: secret_header_value
                    .filter(|v| !v.trim().is_empty())
                    .map(SecretString::new),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub config: ChannelConfigInput,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
    pub name: Option<String>,
    pub config: Option<ChannelConfigInput>,
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ChannelDto {
    pub id: String,
    pub name: String,
    pub channel_type: ChannelType,
    pub config: MaskedChannelConfig,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NotificationChannel> for ChannelDto {
    fn from(value: NotificationChannel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            channel_type: value.channel_type,
            config: value.config.masked(),
            is_enabled: value.is_enabled,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TestChannelResponse {
    pub status: &'static str,
}
