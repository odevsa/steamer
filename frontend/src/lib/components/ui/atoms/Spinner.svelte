<script lang="ts">
  import { LoaderCircle } from 'lucide-svelte';
  import Icon from './Icon.svelte';

  let {
    size = 16,
    class: className = ''
  }: {
    size?: number;
    class?: string;
  } = $props();
</script>

<Icon icon={LoaderCircle} {size} class={`animate-spin ${className}`} />
