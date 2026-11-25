<script lang="ts">
  import { apiClient, ApiClientError } from '$lib/api/client';
  import type { PriceHistoryPoint, WatchlistItem } from '$lib/api/types';
  import * as m from '$lib/paraglide/messages';
  import { localeStore } from '$lib/stores/locale.svelte';
  import { formatCents, formatDateTime } from '$lib/utils/format';
  import Badge from '../atoms/Badge.svelte';
  import Modal from '../atoms/Modal.svelte';
  import Spinner from '../atoms/Spinner.svelte';

  let {
    item = null,
    open = $bindable(false)
  }: {
    item: WatchlistItem | null;
    open?: boolean;
  } = $props();

  let points = $state<PriceHistoryPoint[]>([]);
  let loading = $state(false);
  let errorMessage = $state('');

  $effect(() => {
    if (open && item) {
      const targetItem = item;
      loading = true;
      errorMessage = '';
      apiClient
        .listPriceHistory(targetItem.id)
        .then((res) => {
          points = res;
        })
        .catch((err) => {
          errorMessage = err instanceof ApiClientError ? err.message : m.common_error_generic();
        })
        .finally(() => {
          loading = false;
        });
    }
  });
</script>

<Modal
  bind:open
  title={item ? `${m.history_modal_title()} — ${item.title}` : m.history_modal_title()}
  maxWidthClass="max-w-xl"
>
  {#if loading}
    <div class="flex items-center justify-center gap-2 py-8 text-sm text-text-secondary">
      <Spinner size={16} />
      <span>{m.common_loading()}</span>
    </div>
  {:else if errorMessage}
    <div class="rounded-lg border border-danger-fg/30 bg-danger-bg px-3 py-2 text-xs text-danger-fg">
      {errorMessage}
    </div>
  {:else if points.length === 0}
    <p class="py-6 text-center text-sm text-text-secondary">{m.history_empty()}</p>
  {:else if item}
    <div class="max-h-80 overflow-y-auto rounded-lg border border-border-subtle">
      <table class="w-full text-left text-sm">
        <thead class="border-b border-border-subtle bg-surface-elevated text-xs text-text-secondary">
          <tr>
            <th class="px-3 py-2 font-medium">{m.history_recorded_at()}</th>
            <th class="px-3 py-2 font-medium">{m.history_price()}</th>
            <th class="px-3 py-2 font-medium">{m.history_discount()}</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border-subtle">
          {#each points as point (point.id)}
            <tr>
              <td class="px-3 py-2 text-xs text-text-secondary">
                {formatDateTime(point.recorded_at, localeStore.current)}
              </td>
              <td class="px-3 py-2 font-medium text-text-primary">
                {formatCents(point.price_cents, item.currency, localeStore.current)}
              </td>
              <td class="px-3 py-2">
                {#if point.discount_percent > 0}
                  <Badge variant="discount">-{point.discount_percent}%</Badge>
                {:else}
                  <span class="text-xs text-text-secondary">0%</span>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</Modal>
