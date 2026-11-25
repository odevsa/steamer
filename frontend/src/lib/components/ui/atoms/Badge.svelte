<script lang="ts">
  import type { Snippet } from 'svelte';

  type BadgeVariant = 'neutral' | 'discount' | 'target-met' | 'warning' | 'error';

  let {
    variant = 'neutral',
    class: className = '',
    children
  }: {
    variant?: BadgeVariant;
    class?: string;
    children: Snippet;
  } = $props();

  const variantClasses: Record<BadgeVariant, string> = {
    neutral: 'bg-surface-elevated text-text-secondary border border-border-subtle',
    discount: 'bg-discount-bg text-discount-fg',
    'target-met': 'bg-discount-bg text-discount-fg ring-1 ring-discount-fg/30',
    warning: 'bg-warning-bg text-warning-fg',
    error: 'bg-danger-bg text-danger-fg'
  };
</script>

<span
  class={`inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-xs font-semibold ${variantClasses[variant]} ${className}`}
>
  {@render children()}
</span>
