<script lang="ts">
  import type { Icon as LucideIcon } from 'lucide-svelte';

  let {
    icon,
    size = 18,
    strokeWidth = 2,
    class: className = ''
  }: {
    icon: typeof LucideIcon;
    size?: number;
    strokeWidth?: number;
    class?: string;
  } = $props();

  const IconComponent = $derived(icon);
</script>

<IconComponent {size} {strokeWidth} class={className} aria-hidden="true" />
