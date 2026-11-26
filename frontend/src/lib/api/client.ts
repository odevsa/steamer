import type {
  AlertLog,
  ApiErrorEnvelope,
  ChannelConfigInput,
  GameSearchItem,
  HealthResponse,
  NotificationChannel,
  PriceHistoryPoint,
  SettingsResponse,
  WatchlistItem
} from './types';

export class ApiClientError extends Error {
  readonly status: number;
  readonly code: string;

  constructor(status: number, code: string, message: string) {
    super(message);
    this.status = status;
    this.code = code;
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(path, {
    ...init,
    headers: {
      'content-type': 'application/json',
      ...init?.headers
    }
  });

  if (!response.ok) {
    let code = 'http_error';
    let message = response.statusText || 'Request failed';
    try {
      const parsed = (await response.json()) as Partial<ApiErrorEnvelope>;
      if (parsed.error) {
        code = parsed.error.code || code;
        message = parsed.error.message || message;
      }
    } catch {}
    throw new ApiClientError(response.status, code, message);
  }

  if (response.status === 204) {
    return undefined as T;
  }

  return (await response.json()) as T;
}

export const apiClient = {
  health(): Promise<HealthResponse> {
    return request<HealthResponse>('/api/v1/health');
  },

  searchGames(query: string): Promise<GameSearchItem[]> {
    const params = new URLSearchParams({ q: query });
    return request<GameSearchItem[]>(`/api/v1/steam/search?${params.toString()}`);
  },

  listWatchlist(): Promise<WatchlistItem[]> {
    return request<WatchlistItem[]>('/api/v1/watchlist');
  },

  addWatchlistItem(app_id: number, target_price_cents: number | null): Promise<WatchlistItem> {
    return request<WatchlistItem>('/api/v1/watchlist', {
      method: 'POST',
      body: JSON.stringify({ app_id, target_price_cents })
    });
  },

  updateWatchlistItem(
    id: string,
    patch: { target_price_cents?: number | null; is_active?: boolean }
  ): Promise<WatchlistItem> {
    return request<WatchlistItem>(`/api/v1/watchlist/${encodeURIComponent(id)}`, {
      method: 'PATCH',
      body: JSON.stringify(patch)
    });
  },

  deleteWatchlistItem(id: string): Promise<void> {
    return request<void>(`/api/v1/watchlist/${encodeURIComponent(id)}`, {
      method: 'DELETE'
    });
  },

  triggerPriceCheck(): Promise<{ status: string }> {
    return request<{ status: string }>('/api/v1/watchlist/check', {
      method: 'POST'
    });
  },

  listPriceHistory(id: string): Promise<PriceHistoryPoint[]> {
    return request<PriceHistoryPoint[]>(`/api/v1/watchlist/${encodeURIComponent(id)}/history`);
  },

  listChannels(): Promise<NotificationChannel[]> {
    return request<NotificationChannel[]>('/api/v1/channels');
  },

  createChannel(payload: {
    name: string;
    config: ChannelConfigInput;
    is_enabled: boolean;
  }): Promise<NotificationChannel> {
    return request<NotificationChannel>('/api/v1/channels', {
      method: 'POST',
      body: JSON.stringify(payload)
    });
  },

  updateChannel(
    id: string,
    payload: { name?: string; config?: ChannelConfigInput; is_enabled?: boolean }
  ): Promise<NotificationChannel> {
    return request<NotificationChannel>(`/api/v1/channels/${encodeURIComponent(id)}`, {
      method: 'PATCH',
      body: JSON.stringify(payload)
    });
  },

  deleteChannel(id: string): Promise<void> {
    return request<void>(`/api/v1/channels/${encodeURIComponent(id)}`, {
      method: 'DELETE'
    });
  },

  testChannel(id: string): Promise<{ status: string }> {
    return request<{ status: string }>(`/api/v1/channels/${encodeURIComponent(id)}/test`, {
      method: 'POST'
    });
  },

  listAlerts(limit = 50, offset = 0): Promise<AlertLog[]> {
    const params = new URLSearchParams({
      limit: String(limit),
      offset: String(offset)
    });
    return request<AlertLog[]>(`/api/v1/alerts?${params.toString()}`);
  },

  getSettings(): Promise<SettingsResponse> {
    return request<SettingsResponse>('/api/v1/settings');
  },

  updateSettings(patch: {
    steam_country_code?: string;
    ui_locale?: string;
    poll_interval_secs?: number;
  }): Promise<SettingsResponse> {
    return request<SettingsResponse>('/api/v1/settings', {
      method: 'PATCH',
      body: JSON.stringify(patch)
    });
  }
};
