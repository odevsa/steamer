---
name: ui-component
description: >-
  Atomic Design workflow for creating or updating Svelte UI components in Steamer.
  Use whenever building or refactoring frontend components, pages, modals, or styles.
---

# UI Component (Atomic Design & Design System)

Enforces strict component reuse, semantic theming, Lucide icons, and Paraglide i18n.

## Steps

1. **Inspect Existing Components First**:
   - List `frontend/src/lib/components/ui/atoms`, `molecules`, `organisms`, and `templates`.
   - Reuse existing components whenever possible. Never create ad-hoc `<button>`, `<input>`, or badge markup inside routes.
2. **Place at the Right Atomic Level**:
   - `atoms/`: Single-purpose primitives (`Button`, `Badge`, `Input`, `Select`, `Switch`, `Icon`, `Spinner`). Wrap `bits-ui` for interactive controls.
   - `molecules/`: Small compositions of atoms (`SearchInput`, `PriceDisplay`, `ThemeToggle`, `LocaleSwitcher`).
   - `organisms/`: Feature-complete blocks (`WatchlistCard`, `GameSearchModal`, `ChannelFormModal`).
   - `templates/`: Page layouts (`AppShell`).
3. **Enforce Styling & Tokens**:
   - Use Tailwind classes mapped to semantic CSS variables defined in `docs/04-frontend-design-system.md`.
   - Never use hardcoded hex colors or raw Tailwind palette colors (e.g., `bg-blue-500`).
   - Use `lucide-svelte` via the `Icon` atom for all icons.
4. **Enforce i18n & Zero Comments**:
   - Import Paraglide messages (`import * as m from '$lib/paraglide/messages'`).
   - Add missing keys to both `en.json` and `pt-br.json`.
   - Leave zero comments in `.svelte` or `.ts` files.
5. **Verify**:
   - Run `pnpm --dir frontend check` and `pnpm --dir frontend lint`.
