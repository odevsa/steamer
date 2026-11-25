<script lang="ts">
  import type { Icon as LucideIcon } from 'lucide-svelte';
  import Icon from '../atoms/Icon.svelte';

  let {
    label,
    value,
    icon,
    highlight = false
  }: {
    label: string;
    value: string | number;
    icon: typeof LucideIcon;
    highlight?: boolean;
  } = $props();
</script>

<div class="rounded-xl border border-border-subtle bg-surface p-4">
  <div class="flex items-center justify-between gap-3">
    <span class="text-xs font-medium text-text-secondary">{label}</span>
    <div
      class={`flex h-8 w-8 items-center justify-center rounded-lg ${highlight ? 'bg-discount-bg text-discount-fg' : 'bg-surface-elevated text-text-secondary'}`}
    >
      <Icon {icon} size={16} />
    </div>
  </div>
  <div class="mt-2 text-2xl font-bold text-text-primary">{value}</div>
</div>
