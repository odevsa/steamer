<script lang="ts">
  import { apiClient, ApiClientError } from '$lib/api/client';
  import type { WatchlistItem } from '$lib/api/types';
  import * as m from '$lib/paraglide/messages';
  import { toastStore } from '$lib/stores/toast.svelte';
  import { formatMajorInput, parseMajorToCents } from '$lib/utils/format';
  import Button from '../atoms/Button.svelte';
  import Input from '../atoms/Input.svelte';
  import Modal from '../atoms/Modal.svelte';

  let {
    item = null,
    open = $bindable(false),
    onUpdated
  }: {
    item: WatchlistItem | null;
    open?: boolean;
    onUpdated: (updated: WatchlistItem) => void;
  } = $props();

  let targetInput = $state('');
  let saving = $state(false);
  let errorMessage = $state('');

  $effect(() => {
    if (open && item) {
      targetInput = formatMajorInput(item.target_price_cents);
      errorMessage = '';
    }
  });

  async function save(targetCents: number | null): Promise<void> {
    if (!item) {
      return;
    }
    saving = true;
    errorMessage = '';
    try {
      const updated = await apiClient.updateWatchlistItem(item.id, {
        target_price_cents: targetCents
      });
      toastStore.push('success', m.item_updated_toast());
      open = false;
      onUpdated(updated);
    } catch (err) {
      errorMessage = err instanceof ApiClientError ? err.message : m.common_error_generic();
    } finally {
      saving = false;
    }
  }

  function submitTarget(): void {
    save(parseMajorToCents(targetInput));
  }
</script>

<Modal
  bind:open
  title={m.edit_target_title()}
  description={item ? m.edit_target_desc({ currency: item.currency }) : ''}
>
  {#if item}
    <div class="flex flex-col gap-4">
      {#if errorMessage}
        <div class="rounded-lg border border-danger-fg/30 bg-danger-bg px-3 py-2 text-xs text-danger-fg">
          {errorMessage}
        </div>
      {/if}

      <Input
        bind:value={targetInput}
        label={`${item.title} (${item.currency})`}
        placeholder={m.target_input_placeholder()}
        hint={m.target_input_hint()}
        inputmode="decimal"
      />

      <div class="flex flex-wrap items-center justify-between gap-2">
        <Button variant="secondary" size="sm" loading={saving} onclick={() => save(null)}>
          {m.action_clear_target()}
        </Button>

        <div class="flex gap-2">
          <Button variant="ghost" onclick={() => (open = false)}>
            {m.action_cancel()}
          </Button>
          <Button variant="primary" loading={saving} onclick={submitTarget}>
            {m.action_save()}
          </Button>
        </div>
      </div>
    </div>
  {/if}
</Modal>
