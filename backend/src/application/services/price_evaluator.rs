use std::sync::Arc;

use chrono::Utc;
use futures::future::join_all;

use crate::domain::entities::{
    AlertDecision, AlertStatus, GamePriceSnapshot, NewAlertLog, PriceAlertPayload, WatchlistItem,
};
use crate::domain::errors::DomainError;
use crate::domain::ports::{
    AlertDispatcher, AlertLogRepository, ChannelRepository, PriceHistoryRepository,
    WatchlistRepository,
};

#[derive(Clone)]
pub struct PriceEvaluator {
    watchlist_repo: Arc<dyn WatchlistRepository>,
    price_history_repo: Arc<dyn PriceHistoryRepository>,
    channel_repo: Arc<dyn ChannelRepository>,
    alert_log_repo: Arc<dyn AlertLogRepository>,
    dispatcher: Arc<dyn AlertDispatcher>,
}

impl PriceEvaluator {
    pub fn new(
        watchlist_repo: Arc<dyn WatchlistRepository>,
        price_history_repo: Arc<dyn PriceHistoryRepository>,
        channel_repo: Arc<dyn ChannelRepository>,
        alert_log_repo: Arc<dyn AlertLogRepository>,
        dispatcher: Arc<dyn AlertDispatcher>,
    ) -> Self {
        Self {
            watchlist_repo,
            price_history_repo,
            channel_repo,
            alert_log_repo,
            dispatcher,
        }
    }

    pub async fn evaluate_item(
        &self,
        item: &mut WatchlistItem,
        snapshot: &GamePriceSnapshot,
    ) -> Result<bool, DomainError> {
        self.price_history_repo
            .record(
                &item.id,
                snapshot.current_price_cents,
                snapshot.original_price_cents,
                snapshot.discount_percent,
            )
            .await?;

        let decision = item.apply_price_snapshot(snapshot);
        let mut alerted = false;

        if let AlertDecision::Trigger {
            triggered_price_cents,
            discount_percent,
        } = decision
        {
            let channels = self.channel_repo.list_enabled().await?;
            if !channels.is_empty() {
                let now = Utc::now();
                let payload = PriceAlertPayload {
                    event: "price_alert".to_string(),
                    app_id: item.app_id,
                    title: item.title.clone(),
                    currency: item.currency.clone(),
                    current_price_cents: snapshot.current_price_cents,
                    original_price_cents: snapshot.original_price_cents,
                    discount_percent,
                    target_price_cents: item.target_price_cents,
                    store_url: format!("https://store.steampowered.com/app/{}", item.app_id),
                    header_image_url: item.header_image_url.clone(),
                    triggered_at: now,
                };

                let dispatch_futures = channels.iter().map(|channel| {
                    let dispatcher = self.dispatcher.clone();
                    let payload = payload.clone();
                    async move {
                        (
                            channel,
                            dispatcher.dispatch(&channel.config, &payload).await,
                        )
                    }
                });

                let results = join_all(dispatch_futures).await;
                let mut any_succeeded = false;

                for (channel, result) in results {
                    let (status, error_summary) = match result {
                        Ok(()) => {
                            any_succeeded = true;
                            (AlertStatus::Sent, None)
                        }
                        Err(err) => (AlertStatus::Failed, Some(err.to_string())),
                    };

                    let log_entry = NewAlertLog {
                        watchlist_item_id: item.id.clone(),
                        channel_id: Some(channel.id.clone()),
                        channel_name: channel.name.clone(),
                        triggered_price_cents,
                        discount_percent,
                        status,
                        error_summary,
                        sent_at: now,
                    };

                    if let Err(log_err) = self.alert_log_repo.insert(&log_entry).await {
                        tracing::error!(error = %log_err, "failed to record alert log");
                    }
                }

                if any_succeeded {
                    item.mark_notified(triggered_price_cents, now);
                    alerted = true;
                }
            }
        }

        self.watchlist_repo.update(item).await?;
        Ok(alerted)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use chrono::Utc;
    use tokio::sync::Mutex;

    use super::PriceEvaluator;
    use crate::domain::entities::{
        AlertLog, ChannelConfig, ChannelType, GamePriceSnapshot, NewAlertLog, NotificationChannel,
        PriceAlertPayload, PriceHistoryPoint, WatchlistItem,
    };
    use crate::domain::errors::DomainError;
    use crate::domain::ports::{
        AlertDispatcher, AlertLogRepository, ChannelRepository, PriceHistoryRepository,
        WatchlistRepository,
    };
    use crate::domain::value_objects::{AppId, MoneyCents, SecretString};

    struct MemWatchlistRepo(Mutex<Vec<WatchlistItem>>);

    #[async_trait]
    impl WatchlistRepository for MemWatchlistRepo {
        async fn list_all(&self) -> Result<Vec<WatchlistItem>, DomainError> {
            Ok(self.0.lock().await.clone())
        }
        async fn list_active(&self) -> Result<Vec<WatchlistItem>, DomainError> {
            Ok(self.0.lock().await.clone())
        }
        async fn find_by_id(&self, id: &str) -> Result<Option<WatchlistItem>, DomainError> {
            Ok(self.0.lock().await.iter().find(|i| i.id == id).cloned())
        }
        async fn find_by_app_id(
            &self,
            app_id: AppId,
        ) -> Result<Option<WatchlistItem>, DomainError> {
            Ok(self
                .0
                .lock()
                .await
                .iter()
                .find(|i| i.app_id == app_id)
                .cloned())
        }
        async fn insert(&self, item: &WatchlistItem) -> Result<(), DomainError> {
            self.0.lock().await.push(item.clone());
            Ok(())
        }
        async fn update(&self, item: &WatchlistItem) -> Result<(), DomainError> {
            let mut list = self.0.lock().await;
            if let Some(slot) = list.iter_mut().find(|i| i.id == item.id) {
                *slot = item.clone();
            }
            Ok(())
        }
        async fn delete(&self, id: &str) -> Result<bool, DomainError> {
            let mut list = self.0.lock().await;
            let len = list.len();
            list.retain(|i| i.id != id);
            Ok(list.len() < len)
        }
    }

    struct MemHistoryRepo(Mutex<Vec<PriceHistoryPoint>>);

    #[async_trait]
    impl PriceHistoryRepository for MemHistoryRepo {
        async fn record(
            &self,
            watchlist_item_id: &str,
            price_cents: MoneyCents,
            original_price_cents: MoneyCents,
            discount_percent: u8,
        ) -> Result<(), DomainError> {
            self.0.lock().await.push(PriceHistoryPoint {
                id: 1,
                watchlist_item_id: watchlist_item_id.to_string(),
                price_cents,
                original_price_cents,
                discount_percent,
                recorded_at: Utc::now(),
            });
            Ok(())
        }
        async fn list_for_item(
            &self,
            watchlist_item_id: &str,
            _limit: usize,
        ) -> Result<Vec<PriceHistoryPoint>, DomainError> {
            Ok(self
                .0
                .lock()
                .await
                .iter()
                .filter(|p| p.watchlist_item_id == watchlist_item_id)
                .cloned()
                .collect())
        }
    }

    struct MemChannelRepo(Vec<NotificationChannel>);

    #[async_trait]
    impl ChannelRepository for MemChannelRepo {
        async fn list_all(&self) -> Result<Vec<NotificationChannel>, DomainError> {
            Ok(self.0.clone())
        }
        async fn list_enabled(&self) -> Result<Vec<NotificationChannel>, DomainError> {
            Ok(self.0.iter().filter(|c| c.is_enabled).cloned().collect())
        }
        async fn find_by_id(&self, id: &str) -> Result<Option<NotificationChannel>, DomainError> {
            Ok(self.0.iter().find(|c| c.id == id).cloned())
        }
        async fn insert(&self, _channel: &NotificationChannel) -> Result<(), DomainError> {
            Ok(())
        }
        async fn update(&self, _channel: &NotificationChannel) -> Result<(), DomainError> {
            Ok(())
        }
        async fn delete(&self, _id: &str) -> Result<bool, DomainError> {
            Ok(false)
        }
    }

    struct MemAlertRepo(Mutex<Vec<NewAlertLog>>);

    #[async_trait]
    impl AlertLogRepository for MemAlertRepo {
        async fn insert(&self, entry: &NewAlertLog) -> Result<(), DomainError> {
            self.0.lock().await.push(entry.clone());
            Ok(())
        }
        async fn list_paginated(
            &self,
            _limit: usize,
            _offset: usize,
        ) -> Result<Vec<AlertLog>, DomainError> {
            Ok(Vec::new())
        }
    }

    struct PartialFailDispatcher;

    #[async_trait]
    impl AlertDispatcher for PartialFailDispatcher {
        async fn dispatch(
            &self,
            config: &ChannelConfig,
            _alert: &PriceAlertPayload,
        ) -> Result<(), DomainError> {
            match config {
                ChannelConfig::Discord { .. } => Ok(()),
                _ => Err(DomainError::ExternalService(
                    "simulated failure".to_string(),
                )),
            }
        }
    }

    #[tokio::test]
    async fn isolates_channel_failures_and_updates_dedup_when_one_succeeds() {
        let mut item = WatchlistItem {
            id: "w-1".to_string(),
            app_id: AppId::new(1091500).expect("valid"),
            title: "Cyberpunk 2077".to_string(),
            header_image_url: "https://example.com/img.jpg".to_string(),
            target_price_cents: Some(MoneyCents::new(3000).expect("valid")),
            currency: "USD".to_string(),
            current_price_cents: None,
            original_price_cents: None,
            discount_percent: 0,
            is_free: false,
            is_active: true,
            last_notified_price_cents: None,
            last_notified_at: None,
            last_checked_at: None,
            created_at: Utc::now(),
        };

        let channels = vec![
            NotificationChannel {
                id: "c-1".to_string(),
                name: "Discord".to_string(),
                channel_type: ChannelType::Discord,
                config: ChannelConfig::Discord {
                    webhook_url: SecretString::new("https://discord.com/api/webhooks/1/a"),
                },
                is_enabled: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            NotificationChannel {
                id: "c-2".to_string(),
                name: "Webhook".to_string(),
                channel_type: ChannelType::Webhook,
                config: ChannelConfig::Webhook {
                    url: SecretString::new("https://example.com/hook"),
                    secret_header_name: None,
                    secret_header_value: None,
                },
                is_enabled: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];

        let watchlist_repo = Arc::new(MemWatchlistRepo(Mutex::new(vec![item.clone()])));
        let history_repo = Arc::new(MemHistoryRepo(Mutex::new(Vec::new())));
        let channel_repo = Arc::new(MemChannelRepo(channels));
        let alert_repo = Arc::new(MemAlertRepo(Mutex::new(Vec::new())));
        let dispatcher = Arc::new(PartialFailDispatcher);

        let evaluator = PriceEvaluator::new(
            watchlist_repo,
            history_repo,
            channel_repo,
            alert_repo.clone(),
            dispatcher,
        );

        let snapshot = GamePriceSnapshot {
            app_id: AppId::new(1091500).expect("valid"),
            title: "Cyberpunk 2077".to_string(),
            header_image_url: "https://example.com/img.jpg".to_string(),
            currency: "USD".to_string(),
            current_price_cents: MoneyCents::new(2999).expect("valid"),
            original_price_cents: MoneyCents::new(5999).expect("valid"),
            discount_percent: 50,
            is_free: false,
            fetched_at: Utc::now(),
        };

        let alerted = evaluator
            .evaluate_item(&mut item, &snapshot)
            .await
            .expect("evaluate");

        assert!(alerted);
        assert_eq!(
            item.last_notified_price_cents,
            Some(MoneyCents::new(2999).expect("valid"))
        );
        assert_eq!(alert_repo.0.lock().await.len(), 2);
    }
}
