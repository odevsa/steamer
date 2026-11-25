use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};

use crate::domain::entities::{ChannelConfig, ChannelType, NotificationChannel};
use crate::domain::errors::DomainError;
use crate::domain::ports::{ChannelRepository, SecretCipher};

#[derive(FromRow)]
struct ChannelRow {
    id: String,
    name: String,
    channel_type: String,
    encrypted_config: String,
    is_enabled: i64,
    created_at: String,
    updated_at: String,
}

pub struct SqliteChannelRepository {
    pool: SqlitePool,
    cipher: Arc<dyn SecretCipher>,
}

impl SqliteChannelRepository {
    pub fn new(pool: SqlitePool, cipher: Arc<dyn SecretCipher>) -> Self {
        Self { pool, cipher }
    }

    fn decode_row(&self, row: ChannelRow) -> Result<NotificationChannel, DomainError> {
        let channel_type = ChannelType::parse(&row.channel_type)?;
        let decrypted_json = self.cipher.decrypt(&row.encrypted_config)?;
        let config: ChannelConfig = serde_json::from_str(&decrypted_json)
            .map_err(|_| DomainError::Storage("corrupted channel config payload".to_string()))?;

        Ok(NotificationChannel {
            id: row.id,
            name: row.name,
            channel_type,
            config,
            is_enabled: row.is_enabled != 0,
            created_at: parse_rfc3339(&row.created_at)?,
            updated_at: parse_rfc3339(&row.updated_at)?,
        })
    }

    fn encrypt_config(&self, config: &ChannelConfig) -> Result<String, DomainError> {
        let json = serde_json::to_string(config)
            .map_err(|_| DomainError::Internal("failed to serialize channel config".to_string()))?;
        self.cipher.encrypt(&json)
    }
}

#[async_trait]
impl ChannelRepository for SqliteChannelRepository {
    async fn list_all(&self) -> Result<Vec<NotificationChannel>, DomainError> {
        let rows: Vec<ChannelRow> = sqlx::query_as(
            "SELECT id, name, channel_type, encrypted_config, is_enabled, created_at, updated_at FROM notification_channels ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to query channels: {e}")))?;
        rows.into_iter().map(|r| self.decode_row(r)).collect()
    }

    async fn list_enabled(&self) -> Result<Vec<NotificationChannel>, DomainError> {
        let rows: Vec<ChannelRow> = sqlx::query_as(
            "SELECT id, name, channel_type, encrypted_config, is_enabled, created_at, updated_at FROM notification_channels WHERE is_enabled = 1 ORDER BY created_at ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to query enabled channels: {e}")))?;
        rows.into_iter().map(|r| self.decode_row(r)).collect()
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<NotificationChannel>, DomainError> {
        let row: Option<ChannelRow> = sqlx::query_as(
            "SELECT id, name, channel_type, encrypted_config, is_enabled, created_at, updated_at FROM notification_channels WHERE id = ?1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to find channel: {e}")))?;
        row.map(|r| self.decode_row(r)).transpose()
    }

    async fn insert(&self, channel: &NotificationChannel) -> Result<(), DomainError> {
        let encrypted = self.encrypt_config(&channel.config)?;
        sqlx::query(
            "INSERT INTO notification_channels (id, name, channel_type, encrypted_config, is_enabled, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )
        .bind(&channel.id)
        .bind(&channel.name)
        .bind(channel.channel_type.as_str())
        .bind(encrypted)
        .bind(i64::from(channel.is_enabled))
        .bind(channel.created_at.to_rfc3339())
        .bind(channel.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to insert channel: {e}")))?;
        Ok(())
    }

    async fn update(&self, channel: &NotificationChannel) -> Result<(), DomainError> {
        let encrypted = self.encrypt_config(&channel.config)?;
        let result = sqlx::query(
            "UPDATE notification_channels SET name = ?1, channel_type = ?2, encrypted_config = ?3, is_enabled = ?4, updated_at = ?5 WHERE id = ?6",
        )
        .bind(&channel.name)
        .bind(channel.channel_type.as_str())
        .bind(encrypted)
        .bind(i64::from(channel.is_enabled))
        .bind(channel.updated_at.to_rfc3339())
        .bind(&channel.id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to update channel: {e}")))?;
        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound("channel not found".to_string()));
        }
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<bool, DomainError> {
        let result = sqlx::query("DELETE FROM notification_channels WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Storage(format!("failed to delete channel: {e}")))?;
        Ok(result.rows_affected() > 0)
    }
}

fn parse_rfc3339(raw: &str) -> Result<DateTime<Utc>, DomainError> {
    DateTime::parse_from_rfc3339(raw)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| DomainError::Storage("invalid timestamp stored in database".to_string()))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;

    use super::SqliteChannelRepository;
    use crate::domain::entities::{ChannelConfig, ChannelType, NotificationChannel};
    use crate::domain::ports::ChannelRepository;
    use crate::domain::value_objects::SecretString;
    use crate::infrastructure::persistence::sqlite_pool::init_in_memory_pool;
    use crate::infrastructure::security::AesGcmSecretCipher;

    #[tokio::test]
    async fn stores_encrypted_config_and_decrypts_on_read() {
        let pool = init_in_memory_pool().await;
        let cipher = Arc::new(AesGcmSecretCipher::from_bytes([3u8; 32]));
        let repo = SqliteChannelRepository::new(pool.clone(), cipher);

        let raw_url = "https://discord.com/api/webhooks/12345/super-secret";
        let channel = NotificationChannel {
            id: "ch-1".to_string(),
            name: "My Discord".to_string(),
            channel_type: ChannelType::Discord,
            config: ChannelConfig::Discord {
                webhook_url: SecretString::new(raw_url),
            },
            is_enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repo.insert(&channel).await.expect("insert channel");

        let stored_cipher: (String,) =
            sqlx::query_as("SELECT encrypted_config FROM notification_channels WHERE id = 'ch-1'")
                .fetch_one(&pool)
                .await
                .expect("query raw row");
        assert!(!stored_cipher.0.contains("super-secret"));

        let loaded = repo
            .find_by_id("ch-1")
            .await
            .expect("find")
            .expect("exists");
        assert_eq!(loaded.config, channel.config);
    }
}
