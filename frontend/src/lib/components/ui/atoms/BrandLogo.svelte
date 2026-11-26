<script lang="ts">
  let {
    class: className = 'h-8 w-8',
    label = ''
  }: {
    class?: string;
    label?: string;
  } = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 512 512"
  fill="none"
  class={className}
  role={label ? 'img' : undefined}
  aria-label={label || undefined}
  aria-hidden={label ? undefined : 'true'}
>
  <defs>
    <mask id="steamer-brand-tag-mask">
      <rect width="512" height="512" fill="white" />
      <path
        d="M 211.60 268.20 L 275.40 334.20 L 328.20 283.60 L 429.40 382.60"
        stroke="black"
        stroke-width="83.6"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
      <path
        d="M 468.72 421.06 L 386.22 417.30 L 463.14 338.67 Z"
        fill="black"
        stroke="black"
        stroke-width="44.0"
        stroke-linejoin="round"
      />
    </mask>
  </defs>

  <g mask="url(#steamer-brand-tag-mask)">
    <path
      class="fill-text-primary transition-colors"
      d="M 51.90 165.20 Q 51.10 152.33 60.22 143.21 L 104.21 99.22 Q 113.33 90.10 126.20 90.90 L 224.37 96.96 Q 239.33 97.88 249.93 108.48 L 376.23 234.78 Q 404.23 262.78 376.23 290.78 L 251.78 415.23 Q 223.78 443.23 195.78 415.23 L 69.48 288.93 Q 58.88 278.33 57.96 263.37 Z M 169.02 177.22 A 30.80 30.80 0 1 0 107.42 177.22 A 30.80 30.80 0 1 0 169.02 177.22 Z"
      fill-rule="evenodd"
    />
  </g>

  <path
    class="stroke-brand-accent"
    d="M 211.60 268.20 L 275.40 334.20 L 328.20 283.60 L 429.40 382.60"
    stroke-width="41.8"
    stroke-linecap="round"
    stroke-linejoin="round"
  />
  <path
    class="fill-brand-accent stroke-brand-accent"
    d="M 468.72 421.06 L 386.22 417.30 L 463.14 338.67 Z"
    stroke-width="17.6"
    stroke-linejoin="round"
  />
</svg>
