<script lang="ts">
  import { Search, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import Icon from '../atoms/Icon.svelte';
  import Input from '../atoms/Input.svelte';

  let {
    value = $bindable(''),
    placeholder = '',
    debounceMs = 300,
    onSearch
  }: {
    value?: string;
    placeholder?: string;
    debounceMs?: number;
    onSearch?: (query: string) => void;
  } = $props();

  let timer: ReturnType<typeof setTimeout> | undefined;

  function scheduleSearch(next: string): void {
    clearTimeout(timer);
    timer = setTimeout(() => {
      onSearch?.(next);
    }, debounceMs);
  }

  function handleInput(): void {
    scheduleSearch(value);
  }

  function clearQuery(): void {
    value = '';
    clearTimeout(timer);
    onSearch?.('');
  }
</script>

<Input bind:value {placeholder} oninput={handleInput}>
  {#snippet prefix()}
    <Icon icon={Search} size={16} />
  {/snippet}
  {#snippet suffix()}
    {#if value.length > 0}
      <button
        type="button"
        onclick={clearQuery}
        aria-label={m.common_close()}
        class="rounded p-0.5 text-text-secondary hover:bg-surface-elevated hover:text-text-primary"
      >
        <Icon icon={X} size={14} />
      </button>
    {/if}
  {/snippet}
</Input>
