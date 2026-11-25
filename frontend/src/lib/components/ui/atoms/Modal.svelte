<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Dialog } from 'bits-ui';
  import { X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import Icon from './Icon.svelte';

  let {
    open = $bindable(false),
    onOpenChange,
    title,
    description,
    maxWidthClass = 'max-w-lg',
    children
  }: {
    open?: boolean;
    onOpenChange?: (next: boolean) => void;
    title: string;
    description?: string;
    maxWidthClass?: string;
    children: Snippet;
  } = $props();
</script>

<Dialog.Root bind:open {onOpenChange}>
  <Dialog.Portal>
    <Dialog.Overlay class="fixed inset-0 z-40 bg-canvas/75 backdrop-blur-xs" />
    <Dialog.Content
      class={`fixed top-1/2 left-1/2 z-50 w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 rounded-xl border border-border-subtle bg-surface p-6 shadow-xl ${maxWidthClass}`}
    >
      <div class="flex items-start justify-between gap-4">
        <div>
          <Dialog.Title class="text-lg font-semibold text-text-primary">
            {title}
          </Dialog.Title>
          {#if description}
            <Dialog.Description class="mt-1 text-sm text-text-secondary">
              {description}
            </Dialog.Description>
          {/if}
        </div>
        <Dialog.Close
          aria-label={m.common_close()}
          class="rounded-lg p-1.5 text-text-secondary transition-colors hover:bg-surface-elevated hover:text-text-primary"
        >
          <Icon icon={X} size={18} />
        </Dialog.Close>
      </div>
      <div class="mt-4">
        {@render children()}
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
