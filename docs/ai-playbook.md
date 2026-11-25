# AI Development Playbook

This playbook defines the mandatory, tool-agnostic workflow for any AI coding assistant or human contributor working on Steamer.

## 0. Tool-Agnostic Principles

- **Single Source of Truth**: `AGENTS.md`, `docs/`, and `.agents/skills/*/SKILL.md` use plain Markdown so any AI assistant (CLI, IDE, or autonomous agent) can read and follow them identically without vendor lock-in.
- **Universal Quality Gates**: Verification relies on standard repository commands (`just check`) and Git hooks (`.githooks/pre-commit`), never on proprietary agent-only runtimes.

## 1. Golden Rules

1. **Read First**: Before editing any area, read the corresponding `docs/0X-*.md` spec.
2. **English Only, Zero Comments**: Write all code, identifiers, and docs in English. Never leave inline or block comments in source files.
3. **Clean Architecture & SOLID**:
   - Domain traits live in `backend/src/domain/ports/`.
   - Use cases live in `backend/src/application/use_cases/` and depend only on traits.
   - HTTP handlers in `presentation/handlers/` only parse DTOs, call a use case, and map the response.
   - Keep every file focused and under ~250 lines where practical; extract helpers/modules rather than growing monoliths.
4. **Reuse before Create (UI)**:
   - Always list `frontend/src/lib/components/ui/` first.
   - Compose pages from `organisms/` and `molecules/`, which in turn compose `atoms/`.
   - Never use raw colors (`bg-blue-500`, `#fff`) — use semantic theme tokens.
   - Never write raw UI text — use `paraglide-js` keys in `messages/en.json` and `messages/pt-br.json`.
5. **Security-First**:
   - Any new external URL input must pass `SsrfGuard`.
   - Any new secret field must use `SecretCipher` and be masked in DTOs.

## 2. Anti-Patterns (Strictly Forbidden)

- Putting `sqlx` queries inside Axum handlers or domain entities.
- Returning plaintext tokens/webhooks in `GET` responses or `tracing` logs.
- Creating one-off styled `<button>` or `<input>` elements in route files instead of using `ui/atoms/`.
- Hardcoding `"USD"` or `"en"` when the system/user setting should be used.
- Skipping `just check` after making code changes.
