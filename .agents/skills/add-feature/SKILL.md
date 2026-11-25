---
name: add-feature
description: >-
  Spec-driven workflow for adding or modifying features in Steamer across
  documentation, Clean Architecture Rust backend, and Atomic Design Svelte frontend.
  Use when implementing any new feature, endpoint, or cross-stack change.
---

# Add Feature (Spec-Driven Workflow)

Follow these steps in exact order. Do not skip the spec update.

## Steps

1. **Read & Update Specs**:
   - Read `docs/01-architecture.md` and `docs/02-data-model-and-api.md`.
   - Update the API contract, data model, or PRD in `docs/` first if behavior changes.
   - Create a new ADR in `docs/adr/` if introducing a new dependency or architectural pattern.
2. **Domain Layer (`backend/src/domain/`)**:
   - Define or update entities, value objects, and port traits (`ports/`).
   - Keep `domain/` free of `sqlx`, `axum`, or `reqwest` imports.
3. **Application Layer (`backend/src/application/`)**:
   - Implement the use case in its own focused file under `use_cases/`.
   - Depend only on `Arc<dyn Port>` traits.
   - Add unit tests using in-memory mock traits.
4. **Infrastructure & Presentation (`backend/src/infrastructure/`, `presentation/`)**:
   - Implement trait adapters (SQLx repo, HTTP client, notifier).
   - Add thin Axum handler + validated DTO in `presentation/`.
   - Ensure secrets are encrypted via `SecretCipher` and masked in responses.
5. **Frontend (`frontend/src/`)**:
   - Add typed API client method in `src/lib/api/`.
   - Add i18n keys to `frontend/messages/en.json` and `frontend/messages/pt-br.json`.
   - Use the `ui-component` skill for any new UI elements.
6. **Verify**:
   - Ensure zero comments exist in any source file.
   - Run `just check` and verify all tests and type checks pass.
