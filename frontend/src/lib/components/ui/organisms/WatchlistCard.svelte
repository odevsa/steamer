<script lang="ts">
  import { ExternalLink, History, Pencil, Trash2 } from 'lucide-svelte';
  import type { WatchlistItem } from '$lib/api/types';
  import * as m from '$lib/paraglide/messages';
  import { localeStore } from '$lib/stores/locale.svelte';
  import { formatCents, formatDateTime } from '$lib/utils/format';
  import Badge from '../atoms/Badge.svelte';
  import Button from '../atoms/Button.svelte';
  import Icon from '../atoms/Icon.svelte';
  import Switch from '../atoms/Switch.svelte';
  import PriceDisplay from '../molecules/PriceDisplay.svelte';

  let {
    item,
    onToggleActive,
    onEditTarget,
    onViewHistory,
    onDelete
  }: {
    item: WatchlistItem;
    onToggleActive: (item: WatchlistItem, active: boolean) => void;
    onEditTarget: (item: WatchlistItem) => void;
    onViewHistory: (item: WatchlistItem) => void;
    onDelete: (item: WatchlistItem) => void;
  } = $props();

  const targetText = $derived(
    item.target_price_cents !== null
      ? m.target_price_label({
          price: formatCents(item.target_price_cents, item.currency, localeStore.current)
        })
      : m.target_any_discount()
  );

  const steamUrl = $derived(`https://store.steampowered.com/app/${item.app_id}`);
</script>

<article
  class={`flex flex-col overflow-hidden rounded-xl border border-border-subtle bg-surface transition-opacity ${item.is_active ? '' : 'opacity-75'}`}
>
  <div class="relative aspect-[460/215] w-full bg-surface-elevated">
    <img
      src={item.header_image_url}
      alt={item.title}
      class="h-full w-full object-cover"
      loading="lazy"
    />
    <div class="absolute top-2.5 right-2.5 flex flex-wrap gap-1.5">
      {#if !item.is_active}
        <Badge variant="warning">{m.badge_paused()}</Badge>
      {/if}
      {#if item.is_target_met && item.is_active}
        <Badge variant="target-met">{m.badge_target_met()}</Badge>
      {/if}
    </div>
  </div>

  <div class="flex flex-1 flex-col justify-between p-4">
    <div>
      <div class="flex items-start justify-between gap-2">
        <h3 class="line-clamp-2 text-base font-semibold text-text-primary">
          {item.title}
        </h3>
        <Switch
          checked={item.is_active}
          onCheckedChange={(checked) => onToggleActive(item, checked)}
          ariaLabel={item.is_active ? m.action_pause() : m.action_resume()}
        />
      </div>

      <div class="mt-3">
        <PriceDisplay
          currentPriceCents={item.current_price_cents}
          originalPriceCents={item.original_price_cents}
          discountPercent={item.discount_percent}
          currency={item.currency}
          isFree={item.is_free}
        />
      </div>

      <p class="mt-2 text-xs text-text-secondary">{targetText}</p>
      <p class="mt-1 text-xs text-text-secondary">
        {item.last_checked_at
          ? m.last_checked({ date: formatDateTime(item.last_checked_at, localeStore.current) })
          : m.never_checked()}
      </p>
    </div>

    <div class="mt-4 flex flex-wrap items-center justify-between gap-1 border-t border-border-subtle pt-3">
      <div class="flex items-center gap-1">
        <Button variant="ghost" size="sm" onclick={() => onEditTarget(item)}>
          <Icon icon={Pencil} size={14} />
          <span>{m.action_edit_target()}</span>
        </Button>
        <Button
          variant="ghost"
          size="sm"
          onclick={() => onViewHistory(item)}
          title={m.action_history()}
          aria-label={m.action_history()}
        >
          <Icon icon={History} size={14} />
        </Button>
        <a
          href={steamUrl}
          target="_blank"
          rel="noopener noreferrer"
          title={m.action_open_steam()}
          aria-label={m.action_open_steam()}
          class="inline-flex h-8 items-center justify-center rounded-lg px-2 text-text-secondary transition-colors hover:bg-surface-elevated hover:text-text-primary"
        >
          <Icon icon={ExternalLink} size={14} />
        </a>
      </div>

      <Button
        variant="ghost"
        size="sm"
        onclick={() => onDelete(item)}
        aria-label={m.action_remove()}
        title={m.action_remove()}
        class="text-danger-fg hover:bg-danger-bg hover:text-danger-fg"
      >
        <Icon icon={Trash2} size={14} />
      </Button>
    </div>
  </div>
</article>
