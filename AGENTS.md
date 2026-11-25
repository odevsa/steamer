# Steamer — AI & Engineering Constitution

Steamer is a self-hosted Steam price tracker built with **Rust (Axum + SQLx/SQLite)** and **SvelteKit (Tailwind CSS + bits-ui + Lucide + Paraglide i18n)**, packaged as a multi-arch Docker image for **UmbrelOS**.

## Core Mandates

1. **Language & Comments**: All code, identifiers, commit messages, and documentation MUST be in **English**. **Do NOT write comments in source code** — rely on clear naming, small functions, and explicit types.
2. **Spec-Driven Development**: Read the relevant `docs/` specification before writing or modifying code. Update the spec and/or add an ADR in `docs/adr/` before introducing architectural changes.
3. **Clean Architecture & SOLID**:
   - Keep `backend/` strictly layered: `domain/` (entities, value objects, repository/notifier traits), `application/` (use cases, services, worker orchestration), `infrastructure/` (SQLx repos, Steam HTTP client, encrypted secrets, notifiers), and `presentation/` (Axum routes, DTOs, static SPA serving).
   - Never place business logic in HTTP handlers or database queries in domain structs.
   - Keep files small and single-purpose. Split modules before they grow large; never use ad-hoc hacks.
4. **Security-First**:
   - Encrypt sensitive credentials (bot tokens, webhook URLs) at rest before persisting to SQLite; mask secrets in all API responses.
   - Validate and sanitize all external inputs and user-provided URLs (SSRF protection against private/loopback/link-local ranges).
   - Never expose internal errors, stack traces, or raw SQL errors in HTTP responses.
   - Use parameterized SQLx queries exclusively.
5. **Frontend Atomic Design & Reuse**:
   - Structure UI under `frontend/src/lib/components/ui/{atoms,molecules,organisms,templates}`.
   - **Reuse before Create**: Always inspect `ui/` before creating any visual element. If a primitive does not exist, build it generically in `ui/` first.
   - Use `bits-ui` for accessible primitives, `lucide-svelte` for icons, Tailwind CSS with semantic CSS theme tokens (no hardcoded hex/rgb colors), and `paraglide-js` for all user-facing strings (no hardcoded text).
   - Default locale and Steam region must detect from OS/browser settings with `en` and `USD` (`US`) fallbacks, both overridable in Settings.

## Documentation Map

- [docs/00-vision-and-prd.md](docs/00-vision-and-prd.md) — Product scope, alert rules, user flows
- [docs/01-architecture.md](docs/01-architecture.md) — Clean Architecture layers, dependency rules, backend/frontend structure
- [docs/02-data-model-and-api.md](docs/02-data-model-and-api.md) — SQLite schema, alert deduplication state, REST API contracts
- [docs/03-steam-and-notifications.md](docs/03-steam-and-notifications.md) — Steam Store API, rate limiting, `Notifier` trait, channel specs
- [docs/04-frontend-design-system.md](docs/04-frontend-design-system.md) — Atomic Design, semantic color tokens, theming, i18n
- [docs/05-docker-umbrel-ci.md](docs/05-docker-umbrel-ci.md) — Multi-stage Dockerfile, UmbrelOS manifest, GitHub Actions
- [docs/06-security-architecture.md](docs/06-security-architecture.md) — Threat model, encryption at rest, SSRF guard, container hardening
- [docs/ai-playbook.md](docs/ai-playbook.md) — AI workflow checklist and anti-patterns

## Skills & Workflows

Activate the matching skill in `.agents/skills/` when performing these tasks:
- `add-feature`: End-to-end spec-first feature implementation
- `ui-component`: Creating or extending Atomic Design UI components
- `db-migration`: Adding or modifying SQLite migrations safely
- `security-review`: Auditing endpoints, secret handling, and SSRF/XSS vectors
- `release-umbrel`: Docker multi-arch build, `justfile`, and UmbrelOS packaging
