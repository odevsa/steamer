<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Popover as BitsPopover } from 'bits-ui';

  let {
    open = $bindable(false),
    onOpenChange,
    align = 'end',
    sideOffset = 6,
    triggerClass = '',
    contentClass = '',
    ariaLabel,
    title,
    trigger,
    children
  }: {
    open?: boolean;
    onOpenChange?: (next: boolean) => void;
    align?: 'start' | 'center' | 'end';
    sideOffset?: number;
    triggerClass?: string;
    contentClass?: string;
    ariaLabel?: string;
    title?: string;
    trigger: Snippet;
    children: Snippet;
  } = $props();
</script>

<BitsPopover.Root bind:open {onOpenChange}>
  <BitsPopover.Trigger
    type="button"
    aria-label={ariaLabel}
    {title}
    class={`inline-flex h-8 items-center justify-center gap-1.5 rounded-lg bg-transparent px-2.5 text-xs font-medium text-text-secondary transition-colors hover:bg-surface-elevated hover:text-text-primary focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent disabled:cursor-not-allowed disabled:opacity-50 ${triggerClass}`}
  >
    {@render trigger()}
  </BitsPopover.Trigger>
  <BitsPopover.Portal>
    <BitsPopover.Content
      {align}
      {sideOffset}
      class={`z-50 overflow-hidden rounded-xl border border-border-subtle bg-surface p-1 shadow-lg outline-none ${contentClass}`}
    >
      {@render children()}
    </BitsPopover.Content>
  </BitsPopover.Portal>
</BitsPopover.Root>
