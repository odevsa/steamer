<script lang="ts">
  import { CircleAlert, CircleCheck, Info, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { toastStore, type ToastVariant } from '$lib/stores/toast.svelte';
  import Icon from '../atoms/Icon.svelte';

  const variantStyles: Record<ToastVariant, string> = {
    success: 'border-discount-fg/30 bg-discount-bg text-discount-fg',
    error: 'border-danger-fg/30 bg-danger-bg text-danger-fg',
    info: 'border-border-subtle bg-surface text-text-primary'
  };

  const variantIcons = {
    success: CircleCheck,
    error: CircleAlert,
    info: Info
  };
</script>

{#if toastStore.items.length > 0}
  <div class="fixed right-4 bottom-4 z-50 flex w-80 max-w-[calc(100vw-2rem)] flex-col gap-2">
    {#each toastStore.items as toast (toast.id)}
      <div
        role="status"
        class={`flex items-start justify-between gap-2.5 rounded-lg border px-3.5 py-2.5 text-sm shadow-lg ${variantStyles[toast.variant]}`}
      >
        <div class="flex items-start gap-2">
          <Icon icon={variantIcons[toast.variant]} size={16} class="mt-0.5 shrink-0" />
          <span>{toast.text}</span>
        </div>
        <button
          type="button"
          onclick={() => toastStore.dismiss(toast.id)}
          aria-label={m.common_close()}
          class="rounded p-0.5 opacity-80 hover:opacity-100"
        >
          <Icon icon={X} size={14} />
        </button>
      </div>
    {/each}
  </div>
{/if}
