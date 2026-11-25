export function formatCents(
  cents: number | null | undefined,
  currency: string | null | undefined,
  locale: string
): string {
  if (cents === null || cents === undefined) {
    return '—';
  }
  const isoCurrency = currency || 'USD';
  try {
    return new Intl.NumberFormat(locale === 'pt-br' ? 'pt-BR' : 'en-US', {
      style: 'currency',
      currency: isoCurrency
    }).format(cents / 100);
  } catch {
    return `${isoCurrency} ${(cents / 100).toFixed(2)}`;
  }
}

export function formatMajorInput(cents: number | null | undefined): string {
  if (cents === null || cents === undefined) {
    return '';
  }
  return (cents / 100).toFixed(2);
}

export function parseMajorToCents(raw: string): number | null {
  const trimmed = raw.trim();
  if (!trimmed) {
    return null;
  }
  const normalized = trimmed.replace(',', '.');
  const parsed = Number(normalized);
  if (!Number.isFinite(parsed) || parsed < 0) {
    return null;
  }
  return Math.round(parsed * 100);
}

export function formatDateTime(iso: string | null | undefined, locale: string): string {
  if (!iso) {
    return '—';
  }
  try {
    return new Intl.DateTimeFormat(locale === 'pt-br' ? 'pt-BR' : 'en-US', {
      dateStyle: 'short',
      timeStyle: 'short'
    }).format(new Date(iso));
  } catch {
    return iso;
  }
}
