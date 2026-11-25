# 00 — Vision & Product Requirements Document (PRD)

## 1. Product Goal

**Steamer** is a lightweight, self-hosted Steam price tracker and deal notification service designed to run inside a single Docker container on **UmbrelOS** (x86_64 and ARM64) or any standard Docker host.

## 2. Core User Flows

### 2.1 Search & Add to Watchlist
1. User enters a game title in the search bar.
2. Backend queries the Steam Store search endpoint using the configured Steam region (`country_code`, e.g., `US`, `BR`, `DE`).
3. UI displays matching games with capsule image, title, Steam `app_id`, current price, original price, and discount percentage.
4. User selects a game and optionally enters a **Target Price** (in the active currency):
   - **Target Price provided** (e.g., `$15.00`): Alert triggers when `current_price_cents <= target_price_cents`.
   - **Target Price omitted (`NULL`)**: Alert triggers on **any active discount** (`discount_percent > 0`).
5. Game is persisted in the watchlist.

### 2.2 Watchlist Management
- View all tracked games with current price, original price, discount badge, target price, and alert status.
- Filter/sort watchlist (All, On Sale, Target Reached, Alphabetical, Biggest Discount).
- Edit target price inline or via modal; pause/resume tracking per item; remove item from watchlist.
- Trigger a manual "Check Prices Now" action (rate-limited to prevent Steam API abuse).

### 2.3 Background Price Monitoring & Alerting
1. A Tokio background scheduler wakes up every `poll_interval_secs` (configurable in Settings, default `3600`s).
2. Fetches fresh price data from Steam Store for all active watchlist items in batches/paced requests.
3. Evaluates alert conditions and **deduplication rules** (see `02-data-model-and-api.md`).
4. Dispatches notifications concurrently to all enabled notification channels (`Discord`, `Telegram`, `ntfy`, `Webhook`).
5. Records price history snapshots and alert delivery logs (success/failure without leaking secret URLs/tokens).

### 2.4 Settings, Region & Localization
- **Steam Region / Currency**: Auto-detected from OS/environment locale (`LC_ALL`/`LANG`) or browser `Accept-Language` on first boot, falling back to `US` (`USD`). User can override in Settings at any time.
- **UI Language (i18n)**: Auto-detected from browser/OS locale, falling back to `en`. User can switch in Settings or header switcher.
- **Theme**: Supports `Light`, `Dark`, and `System` (persisted in local storage and applied without flash of unstyled content).
- **Notification Channels**: Add, edit, enable/disable, test, and delete multiple notification channels independently.

## 3. Non-Functional Requirements

- **No Authentication in MVP**: Relies on UmbrelOS / reverse-proxy access control, while **never exposing secrets** (tokens/webhooks) in API responses or logs.
- **Resource Efficiency**: Idle RAM < 50 MB, single container image < 80 MB, multi-arch (`linux/amd64`, `linux/arm64`).
- **Zero Code Comments**: All source files must remain self-documenting without inline or block comments.
