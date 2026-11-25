<script lang="ts">
  import { Switch as BitsSwitch } from 'bits-ui';

  let {
    checked = $bindable(false),
    onCheckedChange,
    disabled = false,
    ariaLabel
  }: {
    checked?: boolean;
    onCheckedChange?: (next: boolean) => void;
    disabled?: boolean;
    ariaLabel: string;
  } = $props();
</script>

<BitsSwitch.Root
  bind:checked
  {onCheckedChange}
  {disabled}
  aria-label={ariaLabel}
  class="peer inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border border-border-subtle transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-accent data-[state=checked]:bg-accent data-[state=unchecked]:bg-surface-elevated"
>
  <BitsSwitch.Thumb
    class="pointer-events-none block h-4 w-4 rounded-full bg-surface shadow-sm transition-transform data-[state=checked]:translate-x-4 data-[state=unchecked]:translate-x-0.5"
  />
</BitsSwitch.Root>
