import { getLocale, setLocale } from '$lib/paraglide/runtime';

export type SupportedLocale = 'en' | 'pt-br';

export function detectBrowserLocale(): SupportedLocale {
  if (typeof navigator !== 'undefined') {
    const lang = (navigator.language || '').toLowerCase();
    if (lang.startsWith('pt')) {
      return 'pt-br';
    }
  }
  return 'en';
}

export function normalizeAppLocale(raw: string): SupportedLocale {
  const lower = raw.trim().toLowerCase();
  if (lower === 'auto') {
    return detectBrowserLocale();
  }
  if (lower.startsWith('pt')) {
    return 'pt-br';
  }
  return 'en';
}

class LocaleStore {
  current = $state<SupportedLocale>('en');

  init(): void {
    const initial = getLocale() as SupportedLocale;
    this.current = initial === 'pt-br' ? 'pt-br' : 'en';
    this.syncDocumentLang();
  }

  set(locale: SupportedLocale): void {
    this.current = locale;
    setLocale(locale, { reload: false });
    this.syncDocumentLang();
  }

  applySetting(settingLocale: string): void {
    const resolved = normalizeAppLocale(settingLocale);
    this.set(resolved);
  }

  private syncDocumentLang(): void {
    if (typeof document !== 'undefined') {
      document.documentElement.lang = this.current === 'pt-br' ? 'pt-BR' : 'en';
    }
  }
}

export const localeStore = new LocaleStore();
