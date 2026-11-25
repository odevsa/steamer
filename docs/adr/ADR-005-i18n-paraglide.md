# ADR-005: Type-Safe i18n with Paraglide & OS Locale Detection

- **Status**: Accepted
- **Date**: 2026-11-25

## Context
The app must support multiple languages, auto-detecting the user's OS/browser language with English (`en`) as fallback, while preventing missing translation keys.

## Decision
Use `@inlang/paraglide-js` for compile-time, tree-shaken, type-safe i18n message functions, paired with browser/OS locale detection and a user-overridable setting.

## Consequences
- Missing translation keys or typos fail at TypeScript compile time (`svelte-check`).
- Zero raw user-facing strings in `.svelte` templates.
