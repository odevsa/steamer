export type ThemeMode = 'light' | 'dark' | 'system';

const STORAGE_KEY = 'steamer-theme';

function readStoredTheme(): ThemeMode {
  if (typeof localStorage === 'undefined') {
    return 'system';
  }
  const raw = localStorage.getItem(STORAGE_KEY);
  if (raw === 'light' || raw === 'dark' || raw === 'system') {
    return raw;
  }
  return 'system';
}

function applyThemeClass(mode: ThemeMode): void {
  if (typeof document === 'undefined') {
    return;
  }
  const prefersDark =
    typeof window !== 'undefined' &&
    window.matchMedia('(prefers-color-scheme: dark)').matches;
  const isDark = mode === 'dark' || (mode === 'system' && prefersDark);
  document.documentElement.classList.toggle('dark', isDark);
}

class ThemeStore {
  mode = $state<ThemeMode>('system');

  init(): void {
    this.mode = readStoredTheme();
    applyThemeClass(this.mode);
    if (typeof window !== 'undefined') {
      window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
        if (this.mode === 'system') {
          applyThemeClass('system');
        }
      });
    }
  }

  set(mode: ThemeMode): void {
    this.mode = mode;
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(STORAGE_KEY, mode);
    }
    applyThemeClass(mode);
  }

  cycle(): void {
    const order: ThemeMode[] = ['light', 'dark', 'system'];
    const idx = order.indexOf(this.mode);
    this.set(order[(idx + 1) % order.length]);
  }
}

export const themeStore = new ThemeStore();
