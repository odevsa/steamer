import { apiClient } from '$lib/api/client';

class VersionStore {
  current = $state(__APP_VERSION__);

  async refresh(): Promise<void> {
    try {
      const health = await apiClient.health();
      if (health.version) {
        this.current = health.version;
      }
    } catch {}
  }
}

export const versionStore = new VersionStore();
