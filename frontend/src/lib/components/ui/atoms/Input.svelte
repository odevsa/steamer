<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLInputAttributes } from 'svelte/elements';

  let {
    id,
    label,
    hint,
    error,
    value = $bindable(''),
    class: className = '',
    prefix,
    suffix,
    ...rest
  }: {
    id?: string;
    label?: string;
    hint?: string;
    error?: string;
    value?: string;
    class?: string;
    prefix?: Snippet;
    suffix?: Snippet;
  } & Omit<HTMLInputAttributes, 'value' | 'class' | 'prefix'> = $props();

  const fallbackId = `input-${Math.random().toString(36).slice(2, 9)}`;
  const inputId = $derived(id ?? fallbackId);
</script>

<div class={`flex flex-col gap-1.5 ${className}`}>
  {#if label}
    <label for={inputId} class="text-sm font-medium text-text-primary">
      {label}
    </label>
  {/if}
  <div
    class={`flex items-center rounded-lg border bg-surface px-3 transition-colors focus-within:border-accent ${error ? 'border-danger-fg' : 'border-border-subtle'}`}
  >
    {#if prefix}
      <div class="mr-2 flex items-center text-text-secondary">
        {@render prefix()}
      </div>
    {/if}
    <input
      id={inputId}
      bind:value
      class="h-9 w-full bg-transparent text-sm text-text-primary placeholder:text-text-secondary/70 focus:outline-none"
      {...rest}
    />
    {#if suffix}
      <div class="ml-2 flex items-center">
        {@render suffix()}
      </div>
    {/if}
  </div>
  {#if error}
    <p class="text-xs text-danger-fg">{error}</p>
  {:else if hint}
    <p class="text-xs text-text-secondary">{hint}</p>
  {/if}
</div>
