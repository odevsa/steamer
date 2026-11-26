<script module lang="ts">
  import type { Icon as LucideIcon } from 'lucide-svelte';

  export interface PopoverOption<V extends string = string> {
    value: V;
    label: string;
    icon?: typeof LucideIcon;
  }
</script>

<script lang="ts" generics="T extends string">
  import { Check, ChevronDown } from 'lucide-svelte';
  import Icon from '../atoms/Icon.svelte';
  import Popover from '../atoms/Popover.svelte';

  let {
    options,
    value,
    onSelect,
    triggerIcon,
    triggerLabel,
    hideLabelOnMobile = false,
    ariaLabel,
    title
  }: {
    options: PopoverOption<T>[];
    value: T;
    onSelect: (next: T) => void;
    triggerIcon: typeof LucideIcon;
    triggerLabel: string;
    hideLabelOnMobile?: boolean;
    ariaLabel: string;
    title: string;
  } = $props();

  let open = $state(false);

  function handleSelect(next: T): void {
    onSelect(next);
    open = false;
  }
</script>

<Popover bind:open {ariaLabel} {title} contentClass="min-w-44">
  {#snippet trigger()}
    <Icon icon={triggerIcon} size={16} />
    <span class={hideLabelOnMobile ? 'hidden sm:inline' : ''}>{triggerLabel}</span>
    <Icon
      icon={ChevronDown}
      size={13}
      class={`text-text-secondary transition-transform ${open ? 'rotate-180' : ''}`}
    />
  {/snippet}

  <div role="listbox" aria-label={title} class="flex flex-col gap-0.5">
    {#each options as option (option.value)}
      {@const selected = option.value === value}
      <button
        type="button"
        role="option"
        aria-selected={selected}
        onclick={() => handleSelect(option.value)}
        class={`flex w-full cursor-pointer items-center justify-between gap-3 rounded-lg px-2.5 py-2 text-left text-sm transition-colors outline-none hover:bg-surface-elevated focus-visible:bg-surface-elevated ${selected ? 'bg-surface-elevated/70 font-medium text-text-primary' : 'text-text-secondary hover:text-text-primary'}`}
      >
        <span class="flex items-center gap-2 truncate">
          {#if option.icon}
            <Icon
              icon={option.icon}
              size={15}
              class={selected ? 'text-accent' : 'text-text-secondary'}
            />
          {/if}
          <span class="truncate">{option.label}</span>
        </span>
        {#if selected}
          <Icon icon={Check} size={14} class="shrink-0 text-accent" />
        {/if}
      </button>
    {/each}
  </div>
</Popover>
