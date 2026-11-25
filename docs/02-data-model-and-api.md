# 02 — Data Model, Alert State Machine & REST API

## 1. SQLite Schema (`backend/migrations/`)

All monetary values are stored as **integer minor units** (`INTEGER`, e.g., cents) alongside ISO currency code to avoid floating-point rounding bugs.

### `watchlist_items`
| Column | Type | Constraints | Notes |
| :--- | :--- | :--- | :--- |
| `id` | `TEXT` | `PRIMARY KEY` | UUIDv4 |
| `app_id` | `INTEGER` | `NOT NULL UNIQUE` | Steam App ID |
| `title` | `TEXT` | `NOT NULL` | Cached game title |
| `header_image_url` | `TEXT` | `NOT NULL` | Steam capsule URL |
| `target_price_cents` | `INTEGER` | `NULL` | `NULL` = alert on any discount |
| `currency` | `TEXT` | `NOT NULL` | e.g., `USD`, `BRL` |
| `current_price_cents` | `INTEGER` | `NULL` | Latest fetched final price |
| `original_price_cents` | `INTEGER` | `NULL` | Latest fetched base price |
| `discount_percent` | `INTEGER` | `NOT NULL DEFAULT 0` | `0..=100` |
| `is_free` | `INTEGER` | `NOT NULL DEFAULT 0` | Boolean (`0`/`1`) |
| `is_active` | `INTEGER` | `NOT NULL DEFAULT 1` | Pause/resume tracking |
| `last_notified_price_cents` | `INTEGER` | `NULL` | Deduplication state |
| `last_notified_at` | `TEXT` | `NULL` | ISO-8601 UTC |
| `last_checked_at` | `TEXT` | `NULL` | ISO-8601 UTC |
| `created_at` | `TEXT` | `NOT NULL` | ISO-8601 UTC |

### `price_history`
| Column | Type | Notes |
| :--- | :--- | :--- |
| `id` | `INTEGER` | `PRIMARY KEY AUTOINCREMENT` |
| `watchlist_item_id` | `TEXT` | `NOT NULL REFERENCES watchlist_items(id) ON DELETE CASCADE` |
| `price_cents` | `INTEGER` | `NOT NULL` |
| `original_price_cents` | `INTEGER` | `NOT NULL` |
| `discount_percent` | `INTEGER` | `NOT NULL` |
| `recorded_at` | `TEXT` | `NOT NULL` |

### `notification_channels`
| Column | Type | Notes |
| :--- | :--- | :--- |
| `id` | `TEXT` | `PRIMARY KEY` |
| `name` | `TEXT` | `NOT NULL` |
| `channel_type` | `TEXT` | `discord` \| `telegram` \| `ntfy` \| `webhook` |
| `encrypted_config` | `TEXT` | `NOT NULL` — AES-256-GCM encrypted JSON blob |
| `is_enabled` | `INTEGER` | `NOT NULL DEFAULT 1` |
| `created_at` | `TEXT` | `NOT NULL` |
| `updated_at` | `TEXT` | `NOT NULL` |

### `alert_logs`
| Column | Type | Notes |
| :--- | :--- | :--- |
| `id` | `INTEGER` | `PRIMARY KEY AUTOINCREMENT` |
| `watchlist_item_id` | `TEXT` | `NOT NULL REFERENCES watchlist_items(id) ON DELETE CASCADE` |
| `channel_id` | `TEXT` | `NULL REFERENCES notification_channels(id) ON DELETE SET NULL` |
| `channel_name` | `TEXT` | `NOT NULL` |
| `triggered_price_cents` | `INTEGER` | `NOT NULL` |
| `discount_percent` | `INTEGER` | `NOT NULL` |
| `status` | `TEXT` | `sent` \| `failed` |
| `error_summary` | `TEXT` | `NULL` — sanitized error (never contains secrets/URLs) |
| `sent_at` | `TEXT` | `NOT NULL` |

### `app_settings`
Key-value or single-row table (`id = 1`) storing:
- `steam_country_code` (`TEXT`, e.g., `US`, `BR`)
- `ui_locale` (`TEXT`, e.g., `auto`, `en`, `pt-BR`)
- `poll_interval_secs` (`INTEGER`, default `3600`, min `300`)

## 2. Alert Evaluation & Deduplication State Machine

For an active item (`is_active == true`) with fresh price `(current_price_cents, discount_percent)`:

1. **Condition Check**:
   - If `target_price_cents` is `Some(target)`: condition met iff `current_price_cents <= target`.
   - If `target_price_cents` is `None`: condition met iff `discount_percent > 0`.
2. **Deduplication Check**:
   - If condition is **NOT** met: reset `last_notified_price_cents = NULL` (sale ended / price rose above target, so future drops will notify again).
   - If condition **IS** met:
     - Trigger alert if `last_notified_price_cents` is `None` **OR** `current_price_cents < last_notified_price_cents`.
     - Otherwise, suppress duplicate alert.
3. **Post-Dispatch**:
   - If at least one enabled channel succeeds, update `last_notified_price_cents = Some(current_price_cents)` and `last_notified_at = now`.

## 3. REST API Contract (`/api/v1`)

All endpoints return JSON. Errors use a uniform envelope: `{ "error": { "code": "...", "message": "..." } }`.

| Method | Path | Purpose |
| :--- | :--- | :--- |
| `GET` | `/api/v1/health` | Liveness & DB connectivity check |
| `GET` | `/api/v1/steam/search?q=...` | Search Steam Store using active `country_code` |
| `GET` | `/api/v1/watchlist` | List tracked games + current prices |
| `POST` | `/api/v1/watchlist` | Add game (`app_id`, optional `target_price_cents`) |
| `PATCH` | `/api/v1/watchlist/:id` | Update `target_price_cents` or `is_active` |
| `DELETE` | `/api/v1/watchlist/:id` | Remove game from watchlist |
| `POST` | `/api/v1/watchlist/check` | Trigger immediate price poll (cooldown-guarded) |
| `GET` | `/api/v1/watchlist/:id/history` | Price history points for a tracked game |
| `GET` | `/api/v1/channels` | List channels with **masked** secrets |
| `POST` | `/api/v1/channels` | Create notification channel (SSRF-validated + encrypted) |
| `PATCH` | `/api/v1/channels/:id` | Update channel config or toggle `is_enabled` |
| `DELETE` | `/api/v1/channels/:id` | Delete channel |
| `POST` | `/api/v1/channels/:id/test` | Send a test notification |
| `GET` | `/api/v1/alerts` | Paginated alert delivery logs |
| `GET` | `/api/v1/settings` | Get settings + detected system defaults |
| `PATCH` | `/api/v1/settings` | Update `steam_country_code`, `ui_locale`, `poll_interval_secs` |
