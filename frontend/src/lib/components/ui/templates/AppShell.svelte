<script lang="ts">
  import type { Snippet } from 'svelte';
  import ToastContainer from '../molecules/ToastContainer.svelte';
  import Navbar from '../organisms/Navbar.svelte';

  let { children }: { children: Snippet } = $props();
</script>

<div class="min-h-screen bg-canvas text-text-primary">
  <Navbar />
  <main class="mx-auto max-w-6xl px-4 py-6 sm:px-6">
    {@render children()}
  </main>
  <ToastContainer />
</div>
