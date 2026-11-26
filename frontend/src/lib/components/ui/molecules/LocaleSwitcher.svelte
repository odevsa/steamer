<script lang="ts">
  import { Languages } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { localeStore, type SupportedLocale } from '$lib/stores/locale.svelte';
  import OptionPopover, { type PopoverOption } from './OptionPopover.svelte';

  const options: PopoverOption<SupportedLocale>[] = $derived([
    { value: 'en', label: m.locale_en() },
    { value: 'pt-br', label: m.locale_pt_br() }
  ]);

  const shortLabel = $derived(localeStore.current === 'pt-br' ? 'PT-BR' : 'EN');
</script>

<OptionPopover
  {options}
  value={localeStore.current}
  onSelect={(locale) => localeStore.set(locale)}
  triggerIcon={Languages}
  triggerLabel={shortLabel}
  ariaLabel={`${m.locale_label()}: ${shortLabel}`}
  title={m.locale_label()}
/>
