<script lang="ts">
  import { Languages } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { localeStore, type SupportedLocale } from '$lib/stores/locale.svelte';
  import Button from '../atoms/Button.svelte';
  import Icon from '../atoms/Icon.svelte';

  function toggleLocale(): void {
    const next: SupportedLocale = localeStore.current === 'en' ? 'pt-br' : 'en';
    localeStore.set(next);
  }

  const label = $derived(localeStore.current === 'pt-br' ? 'PT-BR' : 'EN');
</script>

<Button
  variant="ghost"
  size="sm"
  onclick={toggleLocale}
  aria-label={`${m.locale_label()}: ${label}`}
  title={m.locale_label()}
>
  <Icon icon={Languages} size={16} />
  <span>{label}</span>
</Button>
