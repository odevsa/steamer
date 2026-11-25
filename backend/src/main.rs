use std::sync::Arc;

use tokio::sync::watch;
use tracing_subscriber::EnvFilter;

use steamer::application::services::{PriceEvaluator, PriceMonitorWorker, SettingsService};
use steamer::application::use_cases::{
    AlertsUseCase, ChannelsUseCase, SearchGamesUseCase, SettingsUseCase, WatchlistUseCase,
};
use steamer::config::AppConfig;
use steamer::infrastructure::notifications::NotifierRegistry;
use steamer::infrastructure::persistence::{
    init_sqlite_pool, SqliteAlertLogRepository, SqliteChannelRepository, SqliteSettingsRepository,
    SqliteWatchlistRepository,
};
use steamer::infrastructure::security::{AesGcmSecretCipher, SsrfGuard};
use steamer::infrastructure::steam::SteamHttpGateway;
use steamer::presentation::router::build_router;
use steamer::presentation::state::AppState;

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let config = AppConfig::from_env();
    let pool = init_sqlite_pool(&config.database_path())
        .await
        .expect("failed to initialize SQLite database");

    let cipher = Arc::new(
        AesGcmSecretCipher::load_or_create(config.master_key.as_deref(), &config.master_key_path())
            .expect("failed to initialize encryption cipher"),
    );

    let url_validator = Arc::new(SsrfGuard::new(config.allow_http_webhooks));
    let watchlist_repo = Arc::new(SqliteWatchlistRepository::new(pool.clone()));
    let channel_repo = Arc::new(SqliteChannelRepository::new(pool.clone(), cipher));
    let alert_log_repo = Arc::new(SqliteAlertLogRepository::new(pool.clone()));
    let settings_repo = Arc::new(SqliteSettingsRepository::new(pool.clone()));

    let steam_gateway =
        Arc::new(SteamHttpGateway::new().expect("failed to initialize Steam gateway"));
    let dispatcher = Arc::new(
        NotifierRegistry::with_defaults(url_validator.clone())
            .expect("failed to initialize notification registry"),
    );

    let settings_service = SettingsService::new(settings_repo, &config);
    let evaluator = PriceEvaluator::new(
        watchlist_repo.clone(),
        watchlist_repo.clone(),
        channel_repo.clone(),
        alert_log_repo.clone(),
        dispatcher.clone(),
    );
    let worker = PriceMonitorWorker::new(
        watchlist_repo.clone(),
        steam_gateway.clone(),
        settings_service.clone(),
        evaluator,
    );

    let state = AppState {
        pool,
        search_games: SearchGamesUseCase::new(steam_gateway.clone(), settings_service.clone()),
        watchlist: WatchlistUseCase::new(
            watchlist_repo.clone(),
            watchlist_repo,
            steam_gateway,
            settings_service.clone(),
            worker.clone(),
        ),
        channels: ChannelsUseCase::new(channel_repo, url_validator, dispatcher),
        alerts: AlertsUseCase::new(alert_log_repo),
        settings: SettingsUseCase::new(settings_service),
    };

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let worker_handle = tokio::spawn(async move {
        worker.run_loop(shutdown_rx).await;
    });

    let app = build_router(state, Some(&config.frontend_dir));
    let bind_addr = config.bind_addr();
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind TCP listener");
    tracing::info!(address = %bind_addr, "Steamer HTTP server listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_tx))
        .await
        .expect("HTTP server failed");

    let _ = worker_handle.await;
}

async fn shutdown_signal(shutdown_tx: watch::Sender<bool>) {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut sig) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            sig.recv().await;
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    let _ = shutdown_tx.send(true);
}
