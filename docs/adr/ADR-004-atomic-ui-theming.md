# ADR-004: Atomic Design, bits-ui, Lucide, and Semantic Theme Tokens

- **Status**: Accepted
- **Date**: 2026-11-25

## Context
Frontend coherence requires strict component reuse, accessible primitives, and consistent theming across light and dark modes.

## Decision
Adopt Atomic Design under `frontend/src/lib/components/ui/` (`atoms`, `molecules`, `organisms`, `templates`), built on `bits-ui`, `lucide-svelte`, and Tailwind CSS v4 backed by semantic CSS custom properties.

## Consequences
- AI agents must reuse existing `ui/` components before creating new ones.
- Zero hardcoded colors; Light, Dark, and System themes work uniformly across all components.
