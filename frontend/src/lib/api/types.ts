export type ChannelType = 'discord' | 'telegram' | 'ntfy' | 'webhook';

export interface GameSearchItem {
  app_id: number;
  title: string;
  header_image_url: string;
  currency: string | null;
  current_price_cents: number | null;
  original_price_cents: number | null;
  discount_percent: number;
  is_free: boolean;
}

export interface WatchlistItem {
  id: string;
  app_id: number;
  title: string;
  header_image_url: string;
  target_price_cents: number | null;
  currency: string;
  current_price_cents: number | null;
  original_price_cents: number | null;
  discount_percent: number;
  is_free: boolean;
  is_active: boolean;
  is_target_met: boolean;
  last_notified_price_cents: number | null;
  last_notified_at: string | null;
  last_checked_at: string | null;
  created_at: string;
}

export interface PriceHistoryPoint {
  id: number;
  watchlist_item_id: string;
  price_cents: number;
  original_price_cents: number;
  discount_percent: number;
  recorded_at: string;
}

export type MaskedChannelConfig =
  | { type: 'discord'; webhook_url: string }
  | { type: 'telegram'; bot_token: string; chat_id: string }
  | { type: 'ntfy'; server_url: string; topic: string; has_access_token: boolean }
  | {
      type: 'webhook';
      url: string;
      secret_header_name: string | null;
      has_secret_header_value: boolean;
    };

export type ChannelConfigInput =
  | { type: 'discord'; webhook_url: string }
  | { type: 'telegram'; bot_token: string; chat_id: string }
  | { type: 'ntfy'; server_url: string; topic: string; access_token?: string }
  | {
      type: 'webhook';
      url: string;
      secret_header_name?: string;
      secret_header_value?: string;
    };

export interface NotificationChannel {
  id: string;
  name: string;
  channel_type: ChannelType;
  config: MaskedChannelConfig;
  is_enabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface AlertLog {
  id: number;
  watchlist_item_id: string;
  channel_id: string | null;
  channel_name: string;
  triggered_price_cents: number;
  discount_percent: number;
  status: 'sent' | 'failed';
  error_summary: string | null;
  sent_at: string;
}

export interface SettingsFields {
  steam_country_code: string;
  ui_locale: string;
  poll_interval_secs: number;
}

export interface SettingsResponse extends SettingsFields {
  detected_defaults: SettingsFields;
}

export interface HealthResponse {
  status: string;
  database: string;
  version: string;
}

export interface ApiErrorEnvelope {
  error: {
    code: string;
    message: string;
  };
}
