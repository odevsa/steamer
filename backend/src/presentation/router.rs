use axum::http::{HeaderName, HeaderValue};
use axum::routing::{get, patch, post};
use axum::Router;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

use crate::presentation::embedded_assets::serve_embedded_spa;
use crate::presentation::handlers::{alerts, channels, health, settings, steam, watchlist};
use crate::presentation::state::AppState;

const CONTENT_SECURITY_POLICY: &str = "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https://shared.akamai.steamstatic.com https://cdn.akamai.steamstatic.com https://store.steampowered.com; connect-src 'self'; frame-ancestors 'self'";

pub fn build_router(state: AppState) -> Router {
    let api = Router::new()
        .route("/health", get(health::health_check))
        .route("/steam/search", get(steam::search_steam_games))
        .route(
            "/watchlist",
            get(watchlist::list_watchlist).post(watchlist::add_watchlist_item),
        )
        .route("/watchlist/check", post(watchlist::trigger_watchlist_check))
        .route(
            "/watchlist/{id}",
            patch(watchlist::update_watchlist_item).delete(watchlist::delete_watchlist_item),
        )
        .route(
            "/watchlist/{id}/history",
            get(watchlist::list_watchlist_history),
        )
        .route(
            "/channels",
            get(channels::list_channels).post(channels::create_channel),
        )
        .route(
            "/channels/{id}",
            patch(channels::update_channel).delete(channels::delete_channel),
        )
        .route("/channels/{id}/test", post(channels::test_channel))
        .route("/alerts", get(alerts::list_alerts))
        .route(
            "/settings",
            get(settings::get_settings).patch(settings::update_settings),
        );

    Router::new()
        .nest("/api/v1", api)
        .with_state(state)
        .fallback(serve_embedded_spa)
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("SAMEORIGIN"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static(CONTENT_SECURITY_POLICY),
        ))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    use super::build_router;
    use crate::application::services::{PriceEvaluator, PriceMonitorWorker, SettingsService};
    use crate::application::use_cases::{
        AlertsUseCase, ChannelsUseCase, SearchGamesUseCase, SettingsUseCase, WatchlistUseCase,
    };
    use crate::config::AppConfig;
    use crate::domain::entities::{GamePriceSnapshot, GameSearchResult};
    use crate::domain::errors::DomainError;
    use crate::domain::ports::SteamGateway;
    use crate::domain::value_objects::{AppId, CountryCode};
    use crate::infrastructure::notifications::NotifierRegistry;
    use crate::infrastructure::persistence::sqlite_pool::init_in_memory_pool;
    use crate::infrastructure::persistence::{
        SqliteAlertLogRepository, SqliteChannelRepository, SqliteSettingsRepository,
        SqliteWatchlistRepository,
    };
    use crate::infrastructure::security::{AesGcmSecretCipher, SsrfGuard};
    use crate::presentation::state::AppState;

    struct StubSteamGateway;

    #[async_trait]
    impl SteamGateway for StubSteamGateway {
        async fn search_games(
            &self,
            _query: &str,
            _country_code: &CountryCode,
        ) -> Result<Vec<GameSearchResult>, DomainError> {
            Ok(Vec::new())
        }

        async fn fetch_app_price(
            &self,
            _app_id: AppId,
            _country_code: &CountryCode,
        ) -> Result<GamePriceSnapshot, DomainError> {
            Err(DomainError::NotFound("not found".to_string()))
        }
    }

    async fn test_state() -> AppState {
        let pool = init_in_memory_pool().await;
        let config = AppConfig::from_env();
        let cipher = Arc::new(AesGcmSecretCipher::from_bytes([5u8; 32]));
        let url_validator = Arc::new(SsrfGuard::new(false));
        let watchlist_repo = Arc::new(SqliteWatchlistRepository::new(pool.clone()));
        let channel_repo = Arc::new(SqliteChannelRepository::new(pool.clone(), cipher));
        let alert_log_repo = Arc::new(SqliteAlertLogRepository::new(pool.clone()));
        let settings_repo = Arc::new(SqliteSettingsRepository::new(pool.clone()));
        let steam_gateway = Arc::new(StubSteamGateway);
        let dispatcher =
            Arc::new(NotifierRegistry::with_defaults(url_validator.clone()).expect("registry"));

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

        AppState {
            pool,
            search_games: SearchGamesUseCase::new(steam_gateway.clone(), settings_service.clone()),
            watchlist: WatchlistUseCase::new(
                watchlist_repo.clone(),
                watchlist_repo,
                steam_gateway,
                settings_service.clone(),
                worker,
            ),
            channels: ChannelsUseCase::new(channel_repo, url_validator, dispatcher),
            alerts: AlertsUseCase::new(alert_log_repo),
            settings: SettingsUseCase::new(settings_service),
        }
    }

    #[tokio::test]
    async fn health_endpoint_returns_ok_and_security_headers() {
        let app = build_router(test_state().await);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("x-content-type-options")
                .and_then(|v| v.to_str().ok()),
            Some("nosniff")
        );
        assert_eq!(
            response
                .headers()
                .get("x-frame-options")
                .and_then(|v| v.to_str().ok()),
            Some("SAMEORIGIN")
        );
        assert!(response.headers().contains_key("content-security-policy"));
        let body = to_bytes(response.into_body(), 16_384).await.expect("bytes");
        let text = String::from_utf8(body.to_vec()).expect("utf8");
        assert!(text.contains(env!("CARGO_PKG_VERSION")));
    }

    #[tokio::test]
    async fn channels_mask_secrets_and_block_ssrf_urls() {
        let state = test_state().await;
        let app = build_router(state);

        let blocked_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/channels")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"name":"Bad","config":{"type":"webhook","url":"https://127.0.0.1/hook"}}"#,
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(blocked_resp.status(), StatusCode::BAD_REQUEST);

        let created_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/channels")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"name":"Discord","config":{"type":"discord","webhook_url":"https://8.8.8.8/api/webhooks/123/secret9999"}}"#,
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(created_resp.status(), StatusCode::CREATED);
        let body = to_bytes(created_resp.into_body(), 16_384)
            .await
            .expect("bytes");
        let text = String::from_utf8(body.to_vec()).expect("utf8");
        assert!(!text.contains("secret9999"));
        assert!(text.contains("••••9999"));
    }

    #[tokio::test]
    async fn api_unknown_route_returns_404_instead_of_spa_fallback() {
        let app = build_router(test_state().await);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/unknown-endpoint")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
