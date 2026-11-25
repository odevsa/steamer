# 03 — Steam Integration & Notification Engine

## 1. Steam Store Gateway (`SteamGateway` Trait)

### Endpoints Used
1. **Search**: `GET https://store.steampowered.com/api/storesearch/?term={query}&l=english&cc={country_code}`
   - Returns `items[]` with `id` (`app_id`), `name`, `tiny_image`, and optional `price` (`currency`, `initial`, `final`).
2. **App Details**: `GET https://store.steampowered.com/api/appdetails?appids={app_id}&cc={country_code}&filters=price_overview,basic`
   - Returns `price_overview` (`currency`, `initial`, `final`, `discount_percent`, `final_formatted`).
   - Handles `is_free: true` games gracefully (0 cents, 0% discount unless marked otherwise).

### Rate Limiting & Resilience
- Steam Storefront API enforces strict rate limits (~200 requests per 5 minutes).
- `SteamHttpGateway` must pace polling requests (e.g., 1.5s delay between `appdetails` calls) and implement exponential backoff on HTTP `429` or `5xx`.
- HTTP client timeout: `10s` per request.
- Search responses may be short-lived cached in memory (e.g., 60s TTL) to avoid redundant upstream calls while typing.

### Region / Currency Auto-Detection
- On initial boot (`app_settings` empty):
  1. Inspect `STEAMER_DEFAULT_REGION` env var if set.
  2. Otherwise parse OS locale (`LC_ALL`, `LC_MESSAGES`, `LANG`, e.g., `pt_BR.UTF-8` -> `BR`).
  3. Fallback to `US` (`USD`) if unmapped or `C`/`POSIX`.
- User can change `steam_country_code` in Settings (`US`, `BR`, `EU`/`DE`, `GB`, `JP`, `CA`, `AU`, etc.). Changing region triggers an optional price refresh in the new currency.

## 2. Notification Engine (`Notifier` Trait)

```rust
#[async_trait]
pub trait Notifier: Send + Sync {
    fn channel_type(&self) -> ChannelType;
    async fn send(&self, config: &ChannelConfig, alert: &PriceAlertPayload) -> Result<(), DomainError>;
}
```

### Supported MVP Channels (Multiple Active Simultaneously)

1. **Discord (`discord`)**:
   - Config: `{ "webhook_url": "https://discord.com/api/webhooks/..." }`
   - Sends rich embed with game title, Steam store link, current price, original price, discount %, and target price (if set).
2. **Telegram (`telegram`)**:
   - Config: `{ "bot_token": "...", "chat_id": "..." }`
   - Calls `https://api.telegram.org/bot{token}/sendMessage` with MarkdownV2/HTML formatted alert and inline store link.
3. **ntfy (`ntfy`)**:
   - Config: `{ "server_url": "https://ntfy.sh", "topic": "...", "access_token": "..." }`
   - Sends HTTP `POST` to `{server_url}/{topic}` with `Title`, `Priority`, `Tags` (`moneybag`), and `Click` header pointing to Steam Store URL.
4. **Generic JSON Webhook (`webhook`)**:
   - Config: `{ "url": "https://...", "secret_header_name": "...", "secret_header_value": "..." }`
   - Sends signed/structured JSON payload:
     ```json
     {
       "event": "price_alert",
       "app_id": 1091500,
       "title": "Cyberpunk 2077",
       "currency": "USD",
       "current_price_cents": 2999,
       "original_price_cents": 5999,
       "discount_percent": 50,
       "target_price_cents": 3000,
       "store_url": "https://store.steampowered.com/app/1091500",
       "triggered_at": "2026-11-25T16:00:00Z"
     }
     ```

### Dispatch & Isolation Rules
- Alerts are dispatched to all `is_enabled == true` channels via `futures::future::join_all`.
- Failure in one channel **must never** block or abort delivery to other channels.
- Every delivery attempt is recorded in `alert_logs` with sanitized error messages.
