<script lang="ts">
  import type { AlertLog } from '$lib/api/types';
  import * as m from '$lib/paraglide/messages';
  import { localeStore } from '$lib/stores/locale.svelte';
  import { formatCents, formatDateTime } from '$lib/utils/format';
  import Badge from '../atoms/Badge.svelte';

  let {
    logs
  }: {
    logs: AlertLog[];
  } = $props();
</script>

<div class="overflow-x-auto rounded-xl border border-border-subtle bg-surface">
  <table class="w-full text-left text-sm">
    <thead class="border-b border-border-subtle bg-surface-elevated text-xs text-text-secondary">
      <tr>
        <th class="px-4 py-3 font-medium">{m.alert_col_channel()}</th>
        <th class="px-4 py-3 font-medium">{m.alert_col_price()}</th>
        <th class="px-4 py-3 font-medium">{m.alert_col_discount()}</th>
        <th class="px-4 py-3 font-medium">{m.alert_col_status()}</th>
        <th class="px-4 py-3 font-medium">{m.alert_col_time()}</th>
        <th class="px-4 py-3 font-medium">{m.alert_col_details()}</th>
      </tr>
    </thead>
    <tbody class="divide-y divide-border-subtle">
      {#each logs as entry (entry.id)}
        <tr class="hover:bg-surface-elevated/40">
          <td class="px-4 py-3 font-medium text-text-primary">{entry.channel_name}</td>
          <td class="px-4 py-3 text-text-primary">
            {formatCents(entry.triggered_price_cents, 'USD', localeStore.current)}
          </td>
          <td class="px-4 py-3">
            <Badge variant="discount">-{entry.discount_percent}%</Badge>
          </td>
          <td class="px-4 py-3">
            {#if entry.status === 'sent'}
              <Badge variant="target-met">{m.alert_status_sent()}</Badge>
            {:else}
              <Badge variant="error">{m.alert_status_failed()}</Badge>
            {/if}
          </td>
          <td class="px-4 py-3 text-xs text-text-secondary">
            {formatDateTime(entry.sent_at, localeStore.current)}
          </td>
          <td class="px-4 py-3 text-xs text-text-secondary">
            {entry.error_summary ?? '—'}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
