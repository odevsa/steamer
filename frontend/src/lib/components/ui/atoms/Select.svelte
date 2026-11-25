<script lang="ts">
  import { Select as BitsSelect } from 'bits-ui';
  import { Check, ChevronDown } from 'lucide-svelte';
  import Icon from './Icon.svelte';

  export interface SelectOption {
    value: string;
    label: string;
    disabled?: boolean;
  }

  let {
    items,
    value = $bindable(''),
    onValueChange,
    label,
    placeholder = '',
    disabled = false,
    class: className = ''
  }: {
    items: SelectOption[];
    value?: string;
    onValueChange?: (next: string) => void;
    label?: string;
    placeholder?: string;
    disabled?: boolean;
    class?: string;
  } = $props();

  const selectedLabel = $derived(
    items.find((item) => item.value === value)?.label ?? placeholder
  );
</script>

<div class={`flex flex-col gap-1.5 ${className}`}>
  {#if label}
    <span class="text-sm font-medium text-text-primary">{label}</span>
  {/if}
  <BitsSelect.Root type="single" bind:value {onValueChange} {items} {disabled}>
    <BitsSelect.Trigger
      class="inline-flex h-9 w-full items-center justify-between gap-2 rounded-lg border border-border-subtle bg-surface px-3 text-sm text-text-primary transition-colors hover:bg-surface-elevated focus-visible:outline-2 focus-visible:outline-accent disabled:cursor-not-allowed disabled:opacity-50"
    >
      <span class="truncate">{selectedLabel}</span>
      <Icon icon={ChevronDown} size={15} class="shrink-0 text-text-secondary" />
    </BitsSelect.Trigger>
    <BitsSelect.Portal>
      <BitsSelect.Content
        sideOffset={6}
        class="z-50 min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-lg border border-border-subtle bg-surface p-1 shadow-lg"
      >
        <BitsSelect.Viewport class="max-h-64">
          {#each items as item (item.value)}
            <BitsSelect.Item
              value={item.value}
              label={item.label}
              disabled={item.disabled}
              class="flex cursor-pointer items-center justify-between rounded-md px-2.5 py-2 text-sm text-text-primary outline-none data-[highlighted]:bg-surface-elevated data-[disabled]:opacity-40"
            >
              {#snippet children({ selected })}
                <span class="truncate">{item.label}</span>
                {#if selected}
                  <Icon icon={Check} size={14} class="text-accent" />
                {/if}
              {/snippet}
            </BitsSelect.Item>
          {/each}
        </BitsSelect.Viewport>
      </BitsSelect.Content>
    </BitsSelect.Portal>
  </BitsSelect.Root>
</div>
