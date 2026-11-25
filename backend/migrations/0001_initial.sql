CREATE TABLE IF NOT EXISTS watchlist_items (
    id TEXT PRIMARY KEY NOT NULL,
    app_id INTEGER NOT NULL UNIQUE,
    title TEXT NOT NULL,
    header_image_url TEXT NOT NULL,
    target_price_cents INTEGER,
    currency TEXT NOT NULL,
    current_price_cents INTEGER,
    original_price_cents INTEGER,
    discount_percent INTEGER NOT NULL DEFAULT 0,
    is_free INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_notified_price_cents INTEGER,
    last_notified_at TEXT,
    last_checked_at TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_watchlist_items_is_active
    ON watchlist_items (is_active);

CREATE TABLE IF NOT EXISTS price_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    watchlist_item_id TEXT NOT NULL REFERENCES watchlist_items(id) ON DELETE CASCADE,
    price_cents INTEGER NOT NULL,
    original_price_cents INTEGER NOT NULL,
    discount_percent INTEGER NOT NULL,
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_price_history_item_recorded
    ON price_history (watchlist_item_id, recorded_at);

CREATE TABLE IF NOT EXISTS notification_channels (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    encrypted_config TEXT NOT NULL,
    is_enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS alert_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    watchlist_item_id TEXT NOT NULL REFERENCES watchlist_items(id) ON DELETE CASCADE,
    channel_id TEXT REFERENCES notification_channels(id) ON DELETE SET NULL,
    channel_name TEXT NOT NULL,
    triggered_price_cents INTEGER NOT NULL,
    discount_percent INTEGER NOT NULL,
    status TEXT NOT NULL,
    error_summary TEXT,
    sent_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_alert_logs_sent_at
    ON alert_logs (sent_at DESC);

CREATE TABLE IF NOT EXISTS app_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    steam_country_code TEXT NOT NULL,
    ui_locale TEXT NOT NULL,
    poll_interval_secs INTEGER NOT NULL DEFAULT 3600
);
