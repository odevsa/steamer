<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import Spinner from './Spinner.svelte';

  type Variant = 'primary' | 'secondary' | 'ghost' | 'danger';
  type Size = 'sm' | 'md' | 'lg';

  let {
    variant = 'primary',
    size = 'md',
    loading = false,
    disabled = false,
    type = 'button',
    class: className = '',
    children,
    ...rest
  }: {
    variant?: Variant;
    size?: Size;
    loading?: boolean;
    disabled?: boolean;
    type?: 'button' | 'submit' | 'reset';
    class?: string;
    children: Snippet;
  } & Omit<HTMLButtonAttributes, 'type' | 'disabled' | 'class'> = $props();

  const variantClasses: Record<Variant, string> = {
    primary: 'bg-accent text-on-accent hover:bg-accent-hover shadow-sm',
    secondary:
      'bg-surface-elevated text-text-primary border border-border-subtle hover:bg-border-subtle/50',
    ghost: 'bg-transparent text-text-secondary hover:bg-surface-elevated hover:text-text-primary',
    danger: 'bg-danger-bg text-danger-fg hover:opacity-90 border border-danger-fg/20'
  };

  const sizeClasses: Record<Size, string> = {
    sm: 'h-8 px-2.5 text-xs gap-1.5',
    md: 'h-9 px-3.5 text-sm gap-2',
    lg: 'h-10 px-4 text-sm gap-2'
  };
</script>

<button
  {type}
  disabled={disabled || loading}
  class={`inline-flex items-center justify-center rounded-lg font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent disabled:cursor-not-allowed disabled:opacity-50 ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
  {...rest}
>
  {#if loading}
    <Spinner size={14} />
  {/if}
  {@render children()}
</button>
