<script lang="ts">
  import { Plus } from 'lucide-svelte';
  import { apiClient, ApiClientError } from '$lib/api/client';
  import type { GameSearchItem, WatchlistItem } from '$lib/api/types';
  import * as m from '$lib/paraglide/messages';
  import { toastStore } from '$lib/stores/toast.svelte';
  import { parseMajorToCents } from '$lib/utils/format';
  import Button from '../atoms/Button.svelte';
  import Input from '../atoms/Input.svelte';
  import Modal from '../atoms/Modal.svelte';
  import Spinner from '../atoms/Spinner.svelte';
  import PriceDisplay from '../molecules/PriceDisplay.svelte';
  import SearchInput from '../molecules/SearchInput.svelte';

  let {
    open = $bindable(false),
    onAdded
  }: {
    open?: boolean;
    onAdded: (item: WatchlistItem) => void;
  } = $props();

  let query = $state('');
  let searching = $state(false);
  let results = $state<GameSearchItem[]>([]);
  let selectedGame = $state<GameSearchItem | null>(null);
  let targetInput = $state('');
  let submitting = $state(false);
  let errorMessage = $state('');

  function resetState(): void {
    query = '';
    results = [];
    selectedGame = null;
    targetInput = '';
    errorMessage = '';
    searching = false;
    submitting = false;
  }

  async function handleSearch(term: string): Promise<void> {
    errorMessage = '';
    const trimmed = term.trim();
    if (trimmed.length < 2) {
      results = [];
      return;
    }
    searching = true;
    try {
      results = await apiClient.searchGames(trimmed);
    } catch (err) {
      errorMessage = err instanceof ApiClientError ? err.message : m.common_error_generic();
    } finally {
      searching = false;
    }
  }

  async function handleAdd(): Promise<void> {
    if (!selectedGame) {
      return;
    }
    submitting = true;
    errorMessage = '';
    const targetCents = parseMajorToCents(targetInput);
    try {
      const created = await apiClient.addWatchlistItem(selectedGame.app_id, targetCents);
      toastStore.push('success', m.item_added_toast());
      open = false;
      resetState();
      onAdded(created);
    } catch (err) {
      errorMessage = err instanceof ApiClientError ? err.message : m.common_error_generic();
    } finally {
      submitting = false;
    }
  }
</script>

<Modal
  bind:open
  onOpenChange={(next) => {
    if (!next) {
      resetState();
    }
  }}
  title={m.search_modal_title()}
  description={m.search_modal_desc()}
  maxWidthClass="max-w-2xl"
>
  <div class="flex flex-col gap-4">
    <SearchInput
      bind:value={query}
      placeholder={m.search_placeholder()}
      onSearch={handleSearch}
    />

    {#if errorMessage}
      <div class="rounded-lg border border-danger-fg/30 bg-danger-bg px-3 py-2 text-xs text-danger-fg">
        {errorMessage}
      </div>
    {/if}

    <div class="max-h-72 overflow-y-auto rounded-lg border border-border-subtle bg-canvas">
      {#if searching}
        <div class="flex items-center justify-center gap-2 py-8 text-sm text-text-secondary">
          <Spinner size={16} />
          <span>{m.common_loading()}</span>
        </div>
      {:else if query.trim().length < 2}
        <p class="py-8 text-center text-sm text-text-secondary">{m.search_prompt()}</p>
      {:else if results.length === 0}
        <p class="py-8 text-center text-sm text-text-secondary">{m.search_no_results()}</p>
      {:else}
        <div class="divide-y divide-border-subtle">
          {#each results as game (game.app_id)}
            {@const isSelected = selectedGame?.app_id === game.app_id}
            <button
              type="button"
              onclick={() => {
                selectedGame = game;
              }}
              class={`flex w-full items-center gap-3 p-3 text-left transition-colors ${isSelected ? 'bg-surface-elevated ring-1 ring-inset ring-accent' : 'hover:bg-surface'}`}
            >
              <img
                src={game.header_image_url}
                alt={game.title}
                class="h-12 w-24 shrink-0 rounded object-cover"
                loading="lazy"
              />
              <div class="min-w-0 flex-1">
                <p class="truncate text-sm font-medium text-text-primary">{game.title}</p>
                <div class="mt-1">
                  <PriceDisplay
                    currentPriceCents={game.current_price_cents}
                    originalPriceCents={game.original_price_cents}
                    discountPercent={game.discount_percent}
                    currency={game.currency}
                    isFree={game.is_free}
                  />
                </div>
              </div>
            </button>
          {/each}
        </div>
      {/if}
    </div>

    {#if selectedGame}
      <div class="rounded-lg border border-border-subtle bg-surface-elevated p-4">
        <p class="text-sm font-semibold text-text-primary">{selectedGame.title}</p>
        <div class="mt-3">
          <Input
            bind:value={targetInput}
            label={m.target_input_label()}
            placeholder={m.target_input_placeholder()}
            hint={m.target_input_hint()}
            inputmode="decimal"
          />
        </div>
      </div>
    {/if}

    <div class="flex justify-end gap-2">
      <Button variant="secondary" onclick={() => (open = false)}>
        {m.action_cancel()}
      </Button>
      <Button
        variant="primary"
        disabled={!selectedGame}
        loading={submitting}
        onclick={handleAdd}
      >
        <Plus size={16} />
        {m.action_track_game()}
      </Button>
    </div>
  </div>
</Modal>
