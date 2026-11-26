<script lang="ts">
  import { Monitor, Moon, Sun } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { themeStore, type ThemeMode } from '$lib/stores/theme.svelte';
  import OptionPopover, { type PopoverOption } from './OptionPopover.svelte';

  const options: PopoverOption<ThemeMode>[] = $derived([
    { value: 'light', label: m.theme_light(), icon: Sun },
    { value: 'dark', label: m.theme_dark(), icon: Moon },
    { value: 'system', label: m.theme_system(), icon: Monitor }
  ]);

  const currentOption = $derived(
    options.find((option) => option.value === themeStore.mode) ?? options[2]
  );
</script>

<OptionPopover
  {options}
  value={themeStore.mode}
  onSelect={(mode) => themeStore.set(mode)}
  triggerIcon={currentOption.icon ?? Monitor}
  triggerLabel={currentOption.label}
  hideLabelOnMobile
  ariaLabel={`${m.theme_label()}: ${currentOption.label}`}
  title={m.theme_label()}
/>
