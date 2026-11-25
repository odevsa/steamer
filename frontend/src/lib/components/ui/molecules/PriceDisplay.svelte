<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { localeStore } from '$lib/stores/locale.svelte';
  import { formatCents } from '$lib/utils/format';
  import Badge from '../atoms/Badge.svelte';

  let {
    currentPriceCents,
    originalPriceCents,
    discountPercent,
    currency,
    isFree = false
  }: {
    currentPriceCents: number | null;
    originalPriceCents: number | null;
    discountPercent: number;
    currency: string | null;
    isFree?: boolean;
  } = $props();
</script>

<div class="flex flex-wrap items-center gap-2">
  {#if isFree}
    <Badge variant="neutral">{m.badge_free()}</Badge>
  {:else}
    <span class="text-base font-bold text-text-primary">
      {formatCents(currentPriceCents, currency, localeStore.current)}
    </span>
    {#if discountPercent > 0 && originalPriceCents !== null && originalPriceCents > (currentPriceCents ?? 0)}
      <span class="text-xs text-text-secondary line-through">
        {formatCents(originalPriceCents, currency, localeStore.current)}
      </span>
      <Badge variant="discount">-{discountPercent}%</Badge>
    {/if}
  {/if}
</div>
