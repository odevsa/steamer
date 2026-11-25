<script lang="ts">
  import { onMount } from 'svelte';
  import { Bell, RefreshCw } from 'lucide-svelte';
  import { apiClient, ApiClientError } from '$lib/api/client';
  import type { AlertLog } from '$lib/api/types';
  import Button from '$lib/components/ui/atoms/Button.svelte';
  import Icon from '$lib/components/ui/atoms/Icon.svelte';
  import Spinner from '$lib/components/ui/atoms/Spinner.svelte';
  import EmptyState from '$lib/components/ui/molecules/EmptyState.svelte';
  import AlertLogTable from '$lib/components/ui/organisms/AlertLogTable.svelte';
  import * as m from '$lib/paraglide/messages';
  import { toastStore } from '$lib/stores/toast.svelte';

  let logs = $state<AlertLog[]>([]);
  let loading = $state(true);

  async function loadAlerts(): Promise<void> {
    loading = true;
    try {
      logs = await apiClient.listAlerts(50, 0);
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadAlerts();
  });
</script>

<section class="flex flex-col gap-6">
  <div class="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
    <div>
      <h1 class="text-2xl font-bold text-text-primary">{m.alerts_title()}</h1>
      <p class="mt-1 text-sm text-text-secondary">{m.alerts_subtitle()}</p>
    </div>

    <Button variant="secondary" loading={loading} onclick={loadAlerts}>
      {#if !loading}
        <Icon icon={RefreshCw} size={15} />
      {/if}
      <span>{m.action_refresh()}</span>
    </Button>
  </div>

  {#if loading}
    <div class="flex items-center justify-center gap-2 py-16 text-sm text-text-secondary">
      <Spinner size={18} />
      <span>{m.common_loading()}</span>
    </div>
  {:else if logs.length === 0}
    <EmptyState
      icon={Bell}
      title={m.alerts_empty_title()}
      description={m.alerts_empty_desc()}
    />
  {:else}
    <AlertLogTable {logs} />
  {/if}
</section>
