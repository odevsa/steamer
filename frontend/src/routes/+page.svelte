<script lang="ts">
  import { onMount } from 'svelte';
  import { Gamepad2, Percent, Plus, RefreshCw, Target } from 'lucide-svelte';
  import { apiClient, ApiClientError } from '$lib/api/client';
  import type { WatchlistItem } from '$lib/api/types';
  import Button from '$lib/components/ui/atoms/Button.svelte';
  import Icon from '$lib/components/ui/atoms/Icon.svelte';
  import Select, { type SelectOption } from '$lib/components/ui/atoms/Select.svelte';
  import Spinner from '$lib/components/ui/atoms/Spinner.svelte';
  import EmptyState from '$lib/components/ui/molecules/EmptyState.svelte';
  import StatCard from '$lib/components/ui/molecules/StatCard.svelte';
  import EditTargetModal from '$lib/components/ui/organisms/EditTargetModal.svelte';
  import GameSearchModal from '$lib/components/ui/organisms/GameSearchModal.svelte';
  import PriceHistoryModal from '$lib/components/ui/organisms/PriceHistoryModal.svelte';
  import WatchlistCard from '$lib/components/ui/organisms/WatchlistCard.svelte';
  import * as m from '$lib/paraglide/messages';
  import { toastStore } from '$lib/stores/toast.svelte';

  type FilterMode = 'all' | 'on_sale' | 'target_met';
  type SortMode = 'recent' | 'alpha' | 'discount';

  let items = $state<WatchlistItem[]>([]);
  let loading = $state(true);
  let checkingPrices = $state(false);
  let filterMode = $state<FilterMode>('all');
  let sortMode = $state<SortMode>('recent');

  let searchOpen = $state(false);
  let editOpen = $state(false);
  let historyOpen = $state(false);
  let activeItem = $state<WatchlistItem | null>(null);

  const filterOptions = $derived<SelectOption[]>([
    { value: 'all', label: m.filter_all() },
    { value: 'on_sale', label: m.filter_on_sale() },
    { value: 'target_met', label: m.filter_target_met() }
  ]);

  const sortOptions = $derived<SelectOption[]>([
    { value: 'recent', label: m.sort_recent() },
    { value: 'alpha', label: m.sort_alpha() },
    { value: 'discount', label: m.sort_discount() }
  ]);

  const onSaleCount = $derived(items.filter((i) => i.discount_percent > 0).length);
  const targetMetCount = $derived(items.filter((i) => i.is_target_met && i.is_active).length);

  const visibleItems = $derived.by(() => {
    const filtered = items.filter((item) => {
      if (filterMode === 'on_sale') {
        return item.discount_percent > 0;
      }
      if (filterMode === 'target_met') {
        return item.is_target_met;
      }
      return true;
    });

    return [...filtered].sort((a, b) => {
      if (sortMode === 'alpha') {
        return a.title.localeCompare(b.title);
      }
      if (sortMode === 'discount') {
        return b.discount_percent - a.discount_percent;
      }
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });
  });

  async function loadWatchlist(): Promise<void> {
    loading = true;
    try {
      items = await apiClient.listWatchlist();
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadWatchlist();
  });

  async function handleCheckNow(): Promise<void> {
    checkingPrices = true;
    try {
      await apiClient.triggerPriceCheck();
      toastStore.push('success', m.check_scheduled_toast());
      setTimeout(() => {
        loadWatchlist();
      }, 2500);
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    } finally {
      checkingPrices = false;
    }
  }

  async function handleToggleActive(item: WatchlistItem, active: boolean): Promise<void> {
    try {
      const updated = await apiClient.updateWatchlistItem(item.id, { is_active: active });
      items = items.map((entry) => (entry.id === updated.id ? updated : entry));
      toastStore.push('success', m.item_updated_toast());
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    }
  }

  async function handleDelete(item: WatchlistItem): Promise<void> {
    try {
      await apiClient.deleteWatchlistItem(item.id);
      items = items.filter((entry) => entry.id !== item.id);
      toastStore.push('success', m.item_deleted_toast());
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    }
  }
</script>

<section class="flex flex-col gap-6">
  <div class="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
    <div>
      <h1 class="text-2xl font-bold text-text-primary">{m.watchlist_title()}</h1>
      <p class="mt-1 text-sm text-text-secondary">{m.watchlist_subtitle()}</p>
    </div>

    <div class="flex flex-wrap items-center gap-2">
      <Button
        variant="secondary"
        loading={checkingPrices}
        disabled={items.length === 0}
        onclick={handleCheckNow}
      >
        {#if !checkingPrices}
          <Icon icon={RefreshCw} size={15} />
        {/if}
        <span>{checkingPrices ? m.checking_prices() : m.action_check_prices()}</span>
      </Button>
      <Button variant="primary" onclick={() => (searchOpen = true)}>
        <Icon icon={Plus} size={16} />
        <span>{m.action_add_game()}</span>
      </Button>
    </div>
  </div>

  <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
    <StatCard label={m.stat_tracked()} value={items.length} icon={Gamepad2} />
    <StatCard label={m.stat_on_sale()} value={onSaleCount} icon={Percent} />
    <StatCard
      label={m.stat_target_met()}
      value={targetMetCount}
      icon={Target}
      highlight={targetMetCount > 0}
    />
  </div>

  {#if items.length > 0}
    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-end">
      <div class="w-full sm:w-44">
        <Select
          items={filterOptions}
          value={filterMode}
          onValueChange={(next) => (filterMode = next as FilterMode)}
        />
      </div>
      <div class="w-full sm:w-48">
        <Select
          items={sortOptions}
          value={sortMode}
          onValueChange={(next) => (sortMode = next as SortMode)}
        />
      </div>
    </div>
  {/if}

  {#if loading}
    <div class="flex items-center justify-center gap-2 py-16 text-sm text-text-secondary">
      <Spinner size={18} />
      <span>{m.common_loading()}</span>
    </div>
  {:else if items.length === 0}
    <EmptyState
      icon={Gamepad2}
      title={m.empty_watchlist_title()}
      description={m.empty_watchlist_desc()}
    >
      {#snippet action()}
        <Button variant="primary" onclick={() => (searchOpen = true)}>
          <Icon icon={Plus} size={16} />
          <span>{m.action_add_game()}</span>
        </Button>
      {/snippet}
    </EmptyState>
  {:else if visibleItems.length === 0}
    <EmptyState
      icon={Target}
      title={m.empty_filtered_title()}
      description={m.empty_filtered_desc()}
    >
      {#snippet action()}
        <Button variant="secondary" onclick={() => (filterMode = 'all')}>
          {m.filter_all()}
        </Button>
      {/snippet}
    </EmptyState>
  {:else}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each visibleItems as item (item.id)}
        <WatchlistCard
          {item}
          onToggleActive={handleToggleActive}
          onEditTarget={(target) => {
            activeItem = target;
            editOpen = true;
          }}
          onViewHistory={(target) => {
            activeItem = target;
            historyOpen = true;
          }}
          onDelete={handleDelete}
        />
      {/each}
    </div>
  {/if}
</section>

<GameSearchModal
  bind:open={searchOpen}
  onAdded={(created) => {
    items = [created, ...items];
  }}
/>

<EditTargetModal
  bind:open={editOpen}
  item={activeItem}
  onUpdated={(updated) => {
    items = items.map((entry) => (entry.id === updated.id ? updated : entry));
  }}
/>

<PriceHistoryModal bind:open={historyOpen} item={activeItem} />
