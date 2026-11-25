<script lang="ts">
  import { apiClient, ApiClientError } from '$lib/api/client';
  import type { ChannelConfigInput, ChannelType, NotificationChannel } from '$lib/api/types';
  import * as m from '$lib/paraglide/messages';
  import { toastStore } from '$lib/stores/toast.svelte';
  import Button from '../atoms/Button.svelte';
  import Input from '../atoms/Input.svelte';
  import Modal from '../atoms/Modal.svelte';
  import Select, { type SelectOption } from '../atoms/Select.svelte';
  import Switch from '../atoms/Switch.svelte';

  let {
    channel = null,
    open = $bindable(false),
    onSaved
  }: {
    channel: NotificationChannel | null;
    open?: boolean;
    onSaved: (saved: NotificationChannel, isEdit: boolean) => void;
  } = $props();

  const channelTypeOptions: SelectOption[] = [
    { value: 'discord', label: 'Discord' },
    { value: 'telegram', label: 'Telegram' },
    { value: 'ntfy', label: 'ntfy' },
    { value: 'webhook', label: 'JSON Webhook' }
  ];

  let name = $state('');
  let channelType = $state<ChannelType>('discord');
  let isEnabled = $state(true);

  let discordWebhookUrl = $state('');
  let telegramBotToken = $state('');
  let telegramChatId = $state('');
  let ntfyServerUrl = $state('https://ntfy.sh');
  let ntfyTopic = $state('');
  let ntfyAccessToken = $state('');
  let webhookUrl = $state('');
  let webhookHeaderName = $state('');
  let webhookHeaderValue = $state('');

  let saving = $state(false);
  let errorMessage = $state('');

  $effect(() => {
    if (open) {
      errorMessage = '';
      if (channel) {
        name = channel.name;
        channelType = channel.channel_type;
        isEnabled = channel.is_enabled;
        discordWebhookUrl = '';
        telegramBotToken = '';
        telegramChatId = '';
        ntfyServerUrl = channel.config.type === 'ntfy' ? channel.config.server_url : 'https://ntfy.sh';
        ntfyTopic = channel.config.type === 'ntfy' ? channel.config.topic : '';
        ntfyAccessToken = '';
        webhookUrl = '';
        webhookHeaderName =
          channel.config.type === 'webhook' ? (channel.config.secret_header_name ?? '') : '';
        webhookHeaderValue = '';
      } else {
        name = '';
        channelType = 'discord';
        isEnabled = true;
        discordWebhookUrl = '';
        telegramBotToken = '';
        telegramChatId = '';
        ntfyServerUrl = 'https://ntfy.sh';
        ntfyTopic = '';
        ntfyAccessToken = '';
        webhookUrl = '';
        webhookHeaderName = '';
        webhookHeaderValue = '';
      }
    }
  });

  function buildConfigInput(): ChannelConfigInput | undefined {
    if (channelType === 'discord') {
      if (channel && !discordWebhookUrl.trim()) {
        return undefined;
      }
      return { type: 'discord', webhook_url: discordWebhookUrl.trim() };
    }
    if (channelType === 'telegram') {
      if (channel && !telegramBotToken.trim() && !telegramChatId.trim()) {
        return undefined;
      }
      return {
        type: 'telegram',
        bot_token: telegramBotToken.trim(),
        chat_id: telegramChatId.trim()
      };
    }
    if (channelType === 'ntfy') {
      return {
        type: 'ntfy',
        server_url: ntfyServerUrl.trim(),
        topic: ntfyTopic.trim(),
        access_token: ntfyAccessToken.trim() || undefined
      };
    }
    if (channel && !webhookUrl.trim()) {
      return undefined;
    }
    return {
      type: 'webhook',
      url: webhookUrl.trim(),
      secret_header_name: webhookHeaderName.trim() || undefined,
      secret_header_value: webhookHeaderValue.trim() || undefined
    };
  }

  async function handleSubmit(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    saving = true;
    errorMessage = '';

    try {
      if (channel) {
        const maybeConfig = buildConfigInput();
        const updated = await apiClient.updateChannel(channel.id, {
          name,
          is_enabled: isEnabled,
          ...(maybeConfig ? { config: maybeConfig } : {})
        });
        toastStore.push('success', m.channel_updated_toast());
        open = false;
        onSaved(updated, true);
      } else {
        const config = buildConfigInput();
        if (!config) {
          return;
        }
        const created = await apiClient.createChannel({
          name,
          config,
          is_enabled: isEnabled
        });
        toastStore.push('success', m.channel_created_toast());
        open = false;
        onSaved(created, false);
      }
    } catch (err) {
      errorMessage = err instanceof ApiClientError ? err.message : m.common_error_generic();
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  bind:open
  title={channel ? m.channel_modal_edit_title() : m.channel_modal_add_title()}
  description={m.channel_modal_desc()}
>
  <form onsubmit={handleSubmit} class="flex flex-col gap-4">
    {#if errorMessage}
      <div class="rounded-lg border border-danger-fg/30 bg-danger-bg px-3 py-2 text-xs text-danger-fg">
        {errorMessage}
      </div>
    {/if}

    <Input
      bind:value={name}
      label={m.channel_name_label()}
      placeholder={m.channel_name_placeholder()}
      required
    />

    <Select
      items={channelTypeOptions}
      bind:value={channelType}
      label={m.channel_type_label()}
      disabled={Boolean(channel)}
    />

    {#if channel}
      <p class="text-xs text-text-secondary">{m.channel_secret_edit_hint()}</p>
    {/if}

    {#if channelType === 'discord'}
      <Input
        bind:value={discordWebhookUrl}
        label={m.channel_webhook_url_label()}
        placeholder={channel?.config.type === 'discord' ? channel.config.webhook_url : 'https://discord.com/api/webhooks/...'}
        type="url"
        required={!channel}
      />
    {:else if channelType === 'telegram'}
      <Input
        bind:value={telegramBotToken}
        label={m.channel_bot_token_label()}
        placeholder={channel?.config.type === 'telegram' ? channel.config.bot_token : '123456:ABC...'}
        type="password"
        required={!channel}
      />
      <Input
        bind:value={telegramChatId}
        label={m.channel_chat_id_label()}
        placeholder={channel?.config.type === 'telegram' ? channel.config.chat_id : '-1001234567890'}
        required={!channel}
      />
    {:else if channelType === 'ntfy'}
      <Input
        bind:value={ntfyServerUrl}
        label={m.channel_server_url_label()}
        placeholder="https://ntfy.sh"
        type="url"
        required
      />
      <Input
        bind:value={ntfyTopic}
        label={m.channel_topic_label()}
        placeholder="steamer-deals"
        required
      />
      <Input
        bind:value={ntfyAccessToken}
        label={m.channel_access_token_label()}
        type="password"
      />
    {:else}
      <Input
        bind:value={webhookUrl}
        label={m.channel_webhook_url_label()}
        placeholder={channel?.config.type === 'webhook' ? channel.config.url : 'https://example.com/webhook'}
        type="url"
        required={!channel}
      />
      <Input
        bind:value={webhookHeaderName}
        label={m.channel_header_name_label()}
        placeholder="X-Webhook-Secret"
      />
      <Input
        bind:value={webhookHeaderValue}
        label={m.channel_header_value_label()}
        type="password"
      />
    {/if}

    <div class="flex items-center justify-between rounded-lg border border-border-subtle bg-surface-elevated px-3 py-2.5">
      <span class="text-sm font-medium text-text-primary">{m.channel_enabled()}</span>
      <Switch bind:checked={isEnabled} ariaLabel={m.channel_enabled()} />
    </div>

    <div class="flex justify-end gap-2">
      <Button variant="secondary" onclick={() => (open = false)}>
        {m.action_cancel()}
      </Button>
      <Button type="submit" variant="primary" loading={saving}>
        {m.action_save()}
      </Button>
    </div>
  </form>
</Modal>
