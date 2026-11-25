<script lang="ts">
  import { onMount } from 'svelte';
  import { BellRing, Pencil, Plus, Send, Trash2 } from 'lucide-svelte';
  import { apiClient, ApiClientError } from '$lib/api/client';
  import type { NotificationChannel, SettingsResponse } from '$lib/api/types';
  import Badge from '$lib/components/ui/atoms/Badge.svelte';
  import Button from '$lib/components/ui/atoms/Button.svelte';
  import Icon from '$lib/components/ui/atoms/Icon.svelte';
  import Input from '$lib/components/ui/atoms/Input.svelte';
  import Select, { type SelectOption } from '$lib/components/ui/atoms/Select.svelte';
  import Spinner from '$lib/components/ui/atoms/Spinner.svelte';
  import Switch from '$lib/components/ui/atoms/Switch.svelte';
  import EmptyState from '$lib/components/ui/molecules/EmptyState.svelte';
  import ChannelFormModal from '$lib/components/ui/organisms/ChannelFormModal.svelte';
  import * as m from '$lib/paraglide/messages';
  import { localeStore } from '$lib/stores/locale.svelte';
  import { toastStore } from '$lib/stores/toast.svelte';

  const regionOptions: SelectOption[] = [
    { value: 'US', label: 'United States (USD)' },
    { value: 'BR', label: 'Brazil (BRL)' },
    { value: 'DE', label: 'Germany / EU (EUR)' },
    { value: 'GB', label: 'United Kingdom (GBP)' },
    { value: 'CA', label: 'Canada (CAD)' },
    { value: 'AU', label: 'Australia (AUD)' },
    { value: 'JP', label: 'Japan (JPY)' },
    { value: 'FR', label: 'France (EUR)' },
    { value: 'ES', label: 'Spain (EUR)' },
    { value: 'IT', label: 'Italy (EUR)' },
    { value: 'PT', label: 'Portugal (EUR)' }
  ];

  const localeOptions = $derived<SelectOption[]>([
    { value: 'auto', label: m.locale_auto() },
    { value: 'en', label: m.locale_en() },
    { value: 'pt-BR', label: m.locale_pt_br() }
  ]);

  let loading = $state(true);
  let savingSettings = $state(false);
  let settings = $state<SettingsResponse | null>(null);
  let steamCountryCode = $state('US');
  let uiLocale = $state('auto');
  let pollIntervalInput = $state('3600');

  let channels = $state<NotificationChannel[]>([]);
  let channelModalOpen = $state(false);
  let editingChannel = $state<NotificationChannel | null>(null);
  let testingChannelId = $state<string | null>(null);

  async function loadPage(): Promise<void> {
    loading = true;
    try {
      const [loadedSettings, loadedChannels] = await Promise.all([
        apiClient.getSettings(),
        apiClient.listChannels()
      ]);
      settings = loadedSettings;
      steamCountryCode = loadedSettings.steam_country_code;
      uiLocale = loadedSettings.ui_locale;
      pollIntervalInput = String(loadedSettings.poll_interval_secs);
      channels = loadedChannels;
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadPage();
  });

  async function saveGeneralSettings(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    savingSettings = true;
    const interval = Number.parseInt(pollIntervalInput, 10);
    try {
      const updated = await apiClient.updateSettings({
        steam_country_code: steamCountryCode,
        ui_locale: uiLocale,
        poll_interval_secs: Number.isFinite(interval) ? interval : 3600
      });
      settings = updated;
      localeStore.applySetting(updated.ui_locale);
      toastStore.push('success', m.settings_saved_toast());
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    } finally {
      savingSettings = false;
    }
  }

  async function toggleChannel(channel: NotificationChannel, enabled: boolean): Promise<void> {
    try {
      const updated = await apiClient.updateChannel(channel.id, { is_enabled: enabled });
      channels = channels.map((c) => (c.id === updated.id ? updated : c));
      toastStore.push('success', m.channel_updated_toast());
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    }
  }

  async function handleTestChannel(id: string): Promise<void> {
    testingChannelId = id;
    try {
      await apiClient.testChannel(id);
      toastStore.push('success', m.channel_test_sent_toast());
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    } finally {
      testingChannelId = null;
    }
  }

  async function handleDeleteChannel(id: string): Promise<void> {
    try {
      await apiClient.deleteChannel(id);
      channels = channels.filter((c) => c.id !== id);
      toastStore.push('success', m.channel_deleted_toast());
    } catch (err) {
      const msg = err instanceof ApiClientError ? err.message : m.common_error_generic();
      toastStore.push('error', msg);
    }
  }

  function channelSummary(channel: NotificationChannel): string {
    const cfg = channel.config;
    if (cfg.type === 'discord') {
      return cfg.webhook_url;
    }
    if (cfg.type === 'telegram') {
      return `${cfg.bot_token} (${cfg.chat_id})`;
    }
    if (cfg.type === 'ntfy') {
      return `${cfg.server_url}/${cfg.topic}`;
    }
    return cfg.url;
  }
</script>

<section class="flex flex-col gap-8">
  <div>
    <h1 class="text-2xl font-bold text-text-primary">{m.settings_title()}</h1>
    <p class="mt-1 text-sm text-text-secondary">{m.settings_subtitle()}</p>
  </div>

  {#if loading}
    <div class="flex items-center justify-center gap-2 py-16 text-sm text-text-secondary">
      <Spinner size={18} />
      <span>{m.common_loading()}</span>
    </div>
  {:else}
    <form
      onsubmit={saveGeneralSettings}
      class="rounded-xl border border-border-subtle bg-surface p-6"
    >
      <h2 class="text-lg font-semibold text-text-primary">{m.settings_general_title()}</h2>

      <div class="mt-4 grid grid-cols-1 gap-4 md:grid-cols-3">
        <div>
          <Select
            items={regionOptions}
            bind:value={steamCountryCode}
            label={m.settings_region_label()}
          />
          <p class="mt-1 text-xs text-text-secondary">{m.settings_region_hint()}</p>
        </div>

        <Select
          items={localeOptions}
          bind:value={uiLocale}
          label={m.settings_locale_label()}
        />

        <Input
          bind:value={pollIntervalInput}
          label={m.settings_poll_label()}
          hint={m.settings_poll_hint()}
          type="number"
          min="300"
          step="60"
        />
      </div>

      <div class="mt-6 flex flex-col justify-between gap-3 border-t border-border-subtle pt-4 sm:flex-row sm:items-center">
        {#if settings}
          <p class="text-xs text-text-secondary">
            {m.settings_detected_default({
              region: settings.detected_defaults.steam_country_code,
              locale: settings.detected_defaults.ui_locale
            })}
          </p>
        {/if}

        <Button type="submit" variant="primary" loading={savingSettings}>
          {m.action_save()}
        </Button>
      </div>
    </form>

    <div class="flex flex-col gap-4">
      <div class="flex flex-col justify-between gap-3 sm:flex-row sm:items-center">
        <div>
          <h2 class="text-lg font-semibold text-text-primary">{m.channels_title()}</h2>
          <p class="mt-0.5 text-sm text-text-secondary">{m.channels_subtitle()}</p>
        </div>

        <Button
          variant="primary"
          onclick={() => {
            editingChannel = null;
            channelModalOpen = true;
          }}
        >
          <Icon icon={Plus} size={16} />
          <span>{m.action_add_channel()}</span>
        </Button>
      </div>

      {#if channels.length === 0}
        <EmptyState
          icon={BellRing}
          title={m.channels_empty_title()}
          description={m.channels_empty_desc()}
        >
          {#snippet action()}
            <Button
              variant="primary"
              onclick={() => {
                editingChannel = null;
                channelModalOpen = true;
              }}
            >
              <Icon icon={Plus} size={16} />
              <span>{m.action_add_channel()}</span>
            </Button>
          {/snippet}
        </EmptyState>
      {:else}
        <div class="grid grid-cols-1 gap-3 md:grid-cols-2">
          {#each channels as channel (channel.id)}
            <article class="flex flex-col justify-between gap-4 rounded-xl border border-border-subtle bg-surface p-4">
              <div class="flex items-start justify-between gap-3">
                <div class="min-w-0">
                  <div class="flex flex-wrap items-center gap-2">
                    <h3 class="truncate text-base font-semibold text-text-primary">
                      {channel.name}
                    </h3>
                    <Badge variant="neutral">{channel.channel_type.toUpperCase()}</Badge>
                    {#if channel.is_enabled}
                      <Badge variant="target-met">{m.channel_enabled()}</Badge>
                    {:else}
                      <Badge variant="warning">{m.channel_disabled()}</Badge>
                    {/if}
                  </div>
                  <p class="mt-1.5 truncate font-mono text-xs text-text-secondary">
                    {channelSummary(channel)}
                  </p>
                </div>

                <Switch
                  checked={channel.is_enabled}
                  onCheckedChange={(checked) => toggleChannel(channel, checked)}
                  ariaLabel={channel.name}
                />
              </div>

              <div class="flex flex-wrap items-center justify-end gap-1.5 border-t border-border-subtle pt-3">
                <Button
                  variant="secondary"
                  size="sm"
                  loading={testingChannelId === channel.id}
                  onclick={() => handleTestChannel(channel.id)}
                >
                  {#if testingChannelId !== channel.id}
                    <Icon icon={Send} size={13} />
                  {/if}
                  <span>{m.action_test_channel()}</span>
                </Button>

                <Button
                  variant="ghost"
                  size="sm"
                  onclick={() => {
                    editingChannel = channel;
                    channelModalOpen = true;
                  }}
                >
                  <Icon icon={Pencil} size={13} />
                  <span>{m.action_edit()}</span>
                </Button>

                <Button
                  variant="ghost"
                  size="sm"
                  onclick={() => handleDeleteChannel(channel.id)}
                  aria-label={m.action_remove()}
                  class="text-danger-fg hover:bg-danger-bg hover:text-danger-fg"
                >
                  <Icon icon={Trash2} size={13} />
                </Button>
              </div>
            </article>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</section>

<ChannelFormModal
  bind:open={channelModalOpen}
  channel={editingChannel}
  onSaved={(saved, isEdit) => {
    if (isEdit) {
      channels = channels.map((c) => (c.id === saved.id ? saved : c));
    } else {
      channels = [saved, ...channels];
    }
  }}
/>
