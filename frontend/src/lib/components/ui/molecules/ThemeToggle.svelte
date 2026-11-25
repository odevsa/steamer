<script lang="ts">
  import { Monitor, Moon, Sun } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { themeStore } from '$lib/stores/theme.svelte';
  import Button from '../atoms/Button.svelte';
  import Icon from '../atoms/Icon.svelte';

  const currentLabel = $derived(
    themeStore.mode === 'light'
      ? m.theme_light()
      : themeStore.mode === 'dark'
        ? m.theme_dark()
        : m.theme_system()
  );

  const currentIcon = $derived(
    themeStore.mode === 'light' ? Sun : themeStore.mode === 'dark' ? Moon : Monitor
  );
</script>

<Button
  variant="ghost"
  size="sm"
  onclick={() => themeStore.cycle()}
  aria-label={`${m.theme_label()}: ${currentLabel}`}
  title={`${m.theme_label()}: ${currentLabel}`}
>
  <Icon icon={currentIcon} size={16} />
  <span class="hidden sm:inline">{currentLabel}</span>
</Button>
