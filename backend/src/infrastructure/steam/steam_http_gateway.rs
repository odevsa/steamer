use std::collections::HashMap;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::{Mutex, RwLock};
use url::Url;

use crate::domain::entities::{GamePriceSnapshot, GameSearchResult};
use crate::domain::errors::DomainError;
use crate::domain::ports::SteamGateway;
use crate::domain::value_objects::{AppId, CountryCode, MoneyCents};

const SEARCH_CACHE_TTL: Duration = Duration::from_secs(60);
const MIN_APPDETAILS_INTERVAL: Duration = Duration::from_millis(1500);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RETRY_ATTEMPTS: u8 = 3;

type SearchCache = HashMap<(String, String), (Instant, Vec<GameSearchResult>)>;

pub struct SteamHttpGateway {
    client: Client,
    base_url: String,
    search_cache: RwLock<SearchCache>,
    last_appdetails_at: Mutex<Option<Instant>>,
}

impl SteamHttpGateway {
    pub fn new() -> Result<Self, DomainError> {
        Self::with_base_url("https://store.steampowered.com")
    }

    pub fn with_base_url(base_url: impl Into<String>) -> Result<Self, DomainError> {
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("Steamer/0.1 (+https://github.com/odevsa/steamer)")
            .build()
            .map_err(|e| DomainError::Internal(format!("failed to build HTTP client: {e}")))?;
        Ok(Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            search_cache: RwLock::new(HashMap::new()),
            last_appdetails_at: Mutex::new(None),
        })
    }

    async fn pace_appdetails(&self) {
        let mut guard = self.last_appdetails_at.lock().await;
        if let Some(last) = *guard {
            let elapsed = last.elapsed();
            if elapsed < MIN_APPDETAILS_INTERVAL {
                tokio::time::sleep(MIN_APPDETAILS_INTERVAL - elapsed).await;
            }
        }
        *guard = Some(Instant::now());
    }

    async fn get_json_with_retry(&self, url: Url) -> Result<String, DomainError> {
        let mut backoff = Duration::from_millis(500);
        for attempt in 1..=MAX_RETRY_ATTEMPTS {
            let response = self.client.get(url.clone()).send().await;
            match response {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        return resp.text().await.map_err(|e| {
                            DomainError::ExternalService(format!(
                                "failed to read Steam response body: {e}"
                            ))
                        });
                    }
                    if (status.as_u16() == 429 || status.is_server_error())
                        && attempt < MAX_RETRY_ATTEMPTS
                    {
                        tokio::time::sleep(backoff).await;
                        backoff *= 3;
                        continue;
                    }
                    return Err(DomainError::ExternalService(format!(
                        "Steam API returned HTTP {}",
                        status.as_u16()
                    )));
                }
                Err(err) if attempt < MAX_RETRY_ATTEMPTS => {
                    tracing::warn!(attempt, error = %err, "Steam request failed, retrying");
                    tokio::time::sleep(backoff).await;
                    backoff *= 3;
                }
                Err(err) => {
                    return Err(DomainError::ExternalService(format!(
                        "Steam API request failed: {err}"
                    )));
                }
            }
        }
        Err(DomainError::ExternalService(
            "Steam API request exhausted retries".to_string(),
        ))
    }
}

#[async_trait]
impl SteamGateway for SteamHttpGateway {
    async fn search_games(
        &self,
        query: &str,
        country_code: &CountryCode,
    ) -> Result<Vec<GameSearchResult>, DomainError> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Err(DomainError::Validation(
                "search query cannot be empty".to_string(),
            ));
        }

        let cache_key = (
            trimmed.to_ascii_lowercase(),
            country_code.as_str().to_string(),
        );
        {
            let cache = self.search_cache.read().await;
            if let Some((cached_at, results)) = cache.get(&cache_key) {
                if cached_at.elapsed() < SEARCH_CACHE_TTL {
                    return Ok(results.clone());
                }
            }
        }

        let mut url = Url::parse(&format!("{}/api/storesearch/", self.base_url))
            .map_err(|e| DomainError::Internal(format!("invalid Steam base URL: {e}")))?;
        url.query_pairs_mut()
            .append_pair("term", trimmed)
            .append_pair("l", "english")
            .append_pair("cc", country_code.as_str());

        let body = self.get_json_with_retry(url).await?;
        let results = parse_store_search_response(&body)?;

        let mut cache = self.search_cache.write().await;
        cache.retain(|_, (ts, _)| ts.elapsed() < SEARCH_CACHE_TTL);
        cache.insert(cache_key, (Instant::now(), results.clone()));

        Ok(results)
    }

    async fn fetch_app_price(
        &self,
        app_id: AppId,
        country_code: &CountryCode,
    ) -> Result<GamePriceSnapshot, DomainError> {
        self.pace_appdetails().await;

        let mut url = Url::parse(&format!("{}/api/appdetails", self.base_url))
            .map_err(|e| DomainError::Internal(format!("invalid Steam base URL: {e}")))?;
        url.query_pairs_mut()
            .append_pair("appids", &app_id.to_string())
            .append_pair("cc", country_code.as_str())
            .append_pair("filters", "price_overview,basic");

        let body = self.get_json_with_retry(url).await?;
        parse_app_details_response(app_id, country_code, &body)
    }
}

#[derive(Deserialize)]
struct StoreSearchResponse {
    #[serde(default)]
    items: Vec<StoreSearchItem>,
}

#[derive(Deserialize)]
struct StoreSearchItem {
    id: i64,
    name: String,
    #[serde(default)]
    tiny_image: String,
    price: Option<StoreSearchPrice>,
}

#[derive(Deserialize)]
struct StoreSearchPrice {
    currency: String,
    initial: i64,
    #[serde(rename = "final")]
    final_price: i64,
}

#[derive(Deserialize)]
struct AppDetailsEnvelope {
    success: bool,
    data: Option<AppDetailsData>,
}

#[derive(Deserialize)]
struct AppDetailsData {
    name: String,
    #[serde(default)]
    is_free: bool,
    #[serde(default)]
    header_image: String,
    price_overview: Option<AppDetailsPriceOverview>,
}

#[derive(Deserialize)]
struct AppDetailsPriceOverview {
    currency: String,
    initial: i64,
    #[serde(rename = "final")]
    final_price: i64,
    discount_percent: u8,
}

pub fn parse_store_search_response(raw_json: &str) -> Result<Vec<GameSearchResult>, DomainError> {
    let parsed: StoreSearchResponse = serde_json::from_str(raw_json).map_err(|_| {
        DomainError::ExternalService("unexpected Steam storesearch payload".to_string())
    })?;

    let mut results = Vec::with_capacity(parsed.items.len());
    for item in parsed.items {
        let Ok(app_id) = AppId::new(item.id) else {
            continue;
        };

        let (currency, current_price_cents, original_price_cents, discount_percent, is_free) =
            match item.price {
                Some(p) => {
                    let current = MoneyCents::new(p.final_price.max(0)).ok();
                    let original = MoneyCents::new(p.initial.max(0)).ok();
                    let discount = if p.initial > 0 && p.final_price < p.initial {
                        let pct = ((p.initial - p.final_price) * 100) / p.initial;
                        u8::try_from(pct).unwrap_or(0).min(100)
                    } else {
                        0
                    };
                    (Some(p.currency), current, original, discount, false)
                }
                None => (None, None, None, 0, true),
            };

        let header_image_url = if item.tiny_image.is_empty() {
            format!(
                "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg",
                app_id.value()
            )
        } else {
            item.tiny_image
        };

        results.push(GameSearchResult {
            app_id,
            title: item.name,
            header_image_url,
            currency,
            current_price_cents,
            original_price_cents,
            discount_percent,
            is_free,
        });
    }

    Ok(results)
}

pub fn parse_app_details_response(
    app_id: AppId,
    country_code: &CountryCode,
    raw_json: &str,
) -> Result<GamePriceSnapshot, DomainError> {
    let map: HashMap<String, AppDetailsEnvelope> =
        serde_json::from_str(raw_json).map_err(|_| {
            DomainError::ExternalService("unexpected Steam appdetails payload".to_string())
        })?;

    let envelope = map
        .get(&app_id.to_string())
        .ok_or_else(|| DomainError::NotFound("Steam app not found in response".to_string()))?;

    if !envelope.success {
        return Err(DomainError::NotFound(
            "Steam app does not exist or is unavailable in this region".to_string(),
        ));
    }

    let data = envelope.data.as_ref().ok_or_else(|| {
        DomainError::ExternalService("Steam appdetails missing data object".to_string())
    })?;

    let header_image_url = if data.header_image.is_empty() {
        format!(
            "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg",
            app_id.value()
        )
    } else {
        data.header_image.clone()
    };

    if let Some(price) = &data.price_overview {
        let current = MoneyCents::new(price.final_price.max(0))?;
        let original = MoneyCents::new(price.initial.max(0))?;
        return Ok(GamePriceSnapshot {
            app_id,
            title: data.name.clone(),
            header_image_url,
            currency: price.currency.clone(),
            current_price_cents: current,
            original_price_cents: original,
            discount_percent: price.discount_percent.min(100),
            is_free: data.is_free,
            fetched_at: Utc::now(),
        });
    }

    if data.is_free {
        return Ok(GamePriceSnapshot {
            app_id,
            title: data.name.clone(),
            header_image_url,
            currency: default_currency_for_country(country_code).to_string(),
            current_price_cents: MoneyCents::zero(),
            original_price_cents: MoneyCents::zero(),
            discount_percent: 0,
            is_free: true,
            fetched_at: Utc::now(),
        });
    }

    Err(DomainError::ExternalService(
        "Steam app has no price information available".to_string(),
    ))
}

fn default_currency_for_country(country_code: &CountryCode) -> &'static str {
    match country_code.as_str() {
        "BR" => "BRL",
        "GB" => "GBP",
        "JP" => "JPY",
        "CA" => "CAD",
        "AU" => "AUD",
        "DE" | "FR" | "ES" | "IT" | "NL" | "PT" => "EUR",
        _ => "USD",
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_app_details_response, parse_store_search_response};
    use crate::domain::value_objects::{AppId, CountryCode, MoneyCents};

    #[test]
    fn parses_store_search_with_discounted_and_free_items() {
        let json = r#"{
            "total": 2,
            "items": [
                {
                    "id": 1091500,
                    "name": "Cyberpunk 2077",
                    "tiny_image": "https://cdn.example.com/cp2077.jpg",
                    "price": { "currency": "USD", "initial": 5999, "final": 2999 }
                },
                {
                    "id": 730,
                    "name": "Counter-Strike 2",
                    "tiny_image": "https://cdn.example.com/cs2.jpg"
                }
            ]
        }"#;

        let parsed = parse_store_search_response(json).expect("valid search json");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].discount_percent, 50);
        assert_eq!(
            parsed[0].current_price_cents,
            Some(MoneyCents::new(2999).expect("valid"))
        );
        assert!(parsed[1].is_free);
    }

    #[test]
    fn parses_app_details_paid_and_free_games() {
        let paid_json = r#"{
            "1091500": {
                "success": true,
                "data": {
                    "name": "Cyberpunk 2077",
                    "is_free": false,
                    "header_image": "https://cdn.example.com/header.jpg",
                    "price_overview": {
                        "currency": "BRL",
                        "initial": 19990,
                        "final": 9995,
                        "discount_percent": 50
                    }
                }
            }
        }"#;

        let snapshot = parse_app_details_response(
            AppId::new(1091500).expect("valid"),
            &CountryCode::new("BR").expect("valid"),
            paid_json,
        )
        .expect("paid snapshot");
        assert_eq!(snapshot.currency, "BRL");
        assert_eq!(snapshot.discount_percent, 50);

        let free_json = r#"{
            "730": {
                "success": true,
                "data": {
                    "name": "Counter-Strike 2",
                    "is_free": true,
                    "header_image": "https://cdn.example.com/cs2.jpg"
                }
            }
        }"#;

        let free_snapshot = parse_app_details_response(
            AppId::new(730).expect("valid"),
            &CountryCode::default_us(),
            free_json,
        )
        .expect("free snapshot");
        assert!(free_snapshot.is_free);
        assert_eq!(free_snapshot.current_price_cents, MoneyCents::zero());
    }
}
