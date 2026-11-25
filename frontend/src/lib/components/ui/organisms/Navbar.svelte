<script lang="ts">
  import { page } from '$app/state';
  import { Bell, Gamepad2, Settings, Tag } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import Icon from '../atoms/Icon.svelte';
  import LocaleSwitcher from '../molecules/LocaleSwitcher.svelte';
  import ThemeToggle from '../molecules/ThemeToggle.svelte';

  const navItems = $derived([
    { href: '/', label: m.nav_watchlist(), icon: Tag },
    { href: '/alerts', label: m.nav_alerts(), icon: Bell },
    { href: '/settings', label: m.nav_settings(), icon: Settings }
  ]);
</script>

<header class="sticky top-0 z-30 border-b border-border-subtle bg-surface/90 backdrop-blur-md">
  <div class="mx-auto flex h-14 max-w-6xl items-center justify-between gap-4 px-4 sm:px-6">
    <div class="flex items-center gap-6">
      <a href="/" class="flex items-center gap-2 text-text-primary">
        <div
          class="flex h-8 w-8 items-center justify-center rounded-lg bg-accent text-on-accent"
        >
          <Icon icon={Gamepad2} size={18} />
        </div>
        <span class="text-base font-bold tracking-tight">{m.app_name()}</span>
      </a>

      <nav class="flex items-center gap-1">
        {#each navItems as item (item.href)}
          {@const active = page.url.pathname === item.href}
          <a
            href={item.href}
            class={`inline-flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-sm font-medium transition-colors ${active ? 'bg-surface-elevated text-text-primary' : 'text-text-secondary hover:bg-surface-elevated/60 hover:text-text-primary'}`}
          >
            <Icon icon={item.icon} size={15} />
            <span class="hidden sm:inline">{item.label}</span>
          </a>
        {/each}
      </nav>
    </div>

    <div class="flex items-center gap-1">
      <LocaleSwitcher />
      <ThemeToggle />
    </div>
  </div>
</header>
