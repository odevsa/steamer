<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { Icon as LucideIcon } from 'lucide-svelte';
  import Icon from '../atoms/Icon.svelte';

  let {
    icon,
    title,
    description,
    action
  }: {
    icon: typeof LucideIcon;
    title: string;
    description: string;
    action?: Snippet;
  } = $props();
</script>

<div
  class="flex flex-col items-center justify-center rounded-xl border border-dashed border-border-subtle bg-surface px-6 py-12 text-center"
>
  <div
    class="flex h-12 w-12 items-center justify-center rounded-full bg-surface-elevated text-text-secondary"
  >
    <Icon {icon} size={24} />
  </div>
  <h3 class="mt-4 text-base font-semibold text-text-primary">{title}</h3>
  <p class="mt-1.5 max-w-md text-sm text-text-secondary">{description}</p>
  {#if action}
    <div class="mt-5">
      {@render action()}
    </div>
  {/if}
</div>
