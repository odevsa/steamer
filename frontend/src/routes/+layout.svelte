<script lang="ts">
  import '../app.css';
  import { onMount, type Snippet } from 'svelte';
  import { apiClient } from '$lib/api/client';
  import AppShell from '$lib/components/ui/templates/AppShell.svelte';
  import { localStorageKey } from '$lib/paraglide/runtime';
  import { localeStore } from '$lib/stores/locale.svelte';
  import { themeStore } from '$lib/stores/theme.svelte';
  import { versionStore } from '$lib/stores/version.svelte';

  let { children }: { children: Snippet } = $props();

  onMount(() => {
    themeStore.init();
    localeStore.init();
    versionStore.refresh();

    const hasLocalLocale = Boolean(localStorage.getItem(localStorageKey));
    if (!hasLocalLocale) {
      apiClient
        .getSettings()
        .then((settings) => {
          localeStore.applySetting(settings.ui_locale);
        })
        .catch(() => {});
    }
  });
</script>

{#key localeStore.current}
  <AppShell>
    {@render children()}
  </AppShell>
{/key}
