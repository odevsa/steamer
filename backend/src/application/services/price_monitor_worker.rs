use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{watch, Notify};

use crate::application::services::price_evaluator::PriceEvaluator;
use crate::application::services::settings_service::SettingsService;
use crate::domain::ports::{SteamGateway, WatchlistRepository};

#[derive(Clone)]
pub struct PriceMonitorWorker {
    watchlist_repo: Arc<dyn WatchlistRepository>,
    steam_gateway: Arc<dyn SteamGateway>,
    settings_service: SettingsService,
    evaluator: PriceEvaluator,
    manual_trigger: Arc<Notify>,
}

impl PriceMonitorWorker {
    pub fn new(
        watchlist_repo: Arc<dyn WatchlistRepository>,
        steam_gateway: Arc<dyn SteamGateway>,
        settings_service: SettingsService,
        evaluator: PriceEvaluator,
    ) -> Self {
        Self {
            watchlist_repo,
            steam_gateway,
            settings_service,
            evaluator,
            manual_trigger: Arc::new(Notify::new()),
        }
    }

    pub fn trigger_now(&self) {
        self.manual_trigger.notify_one();
    }

    pub async fn run_cycle(&self) -> (usize, usize) {
        let settings = match self.settings_service.get_or_init().await {
            Ok(s) => s,
            Err(err) => {
                tracing::error!(error = %err, "failed to load settings for price check cycle");
                return (0, 0);
            }
        };

        let items = match self.watchlist_repo.list_active().await {
            Ok(list) => list,
            Err(err) => {
                tracing::error!(error = %err, "failed to load active watchlist items");
                return (0, 0);
            }
        };

        let mut checked = 0usize;
        let mut alerts_triggered = 0usize;

        for mut item in items {
            match self
                .steam_gateway
                .fetch_app_price(item.app_id, &settings.steam_country_code)
                .await
            {
                Ok(snapshot) => {
                    checked += 1;
                    match self.evaluator.evaluate_item(&mut item, &snapshot).await {
                        Ok(true) => alerts_triggered += 1,
                        Ok(false) => {}
                        Err(err) => {
                            tracing::error!(app_id = %item.app_id, error = %err, "failed to evaluate item price");
                        }
                    }
                }
                Err(err) => {
                    tracing::warn!(app_id = %item.app_id, error = %err, "failed to fetch Steam price");
                }
            }
        }

        (checked, alerts_triggered)
    }

    pub async fn run_loop(&self, mut shutdown: watch::Receiver<bool>) {
        loop {
            let interval_secs = self
                .settings_service
                .get_or_init()
                .await
                .map(|s| s.poll_interval_secs)
                .unwrap_or(3600);

            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(interval_secs)) => {
                    let (checked, alerted) = self.run_cycle().await;
                    tracing::info!(checked, alerted, "scheduled price check cycle completed");
                }
                _ = self.manual_trigger.notified() => {
                    let (checked, alerted) = self.run_cycle().await;
                    tracing::info!(checked, alerted, "manual price check cycle completed");
                }
                _ = shutdown.changed() => {
                    if *shutdown.borrow() {
                        tracing::info!("price monitor worker shutting down");
                        break;
                    }
                }
            }
        }
    }
}
