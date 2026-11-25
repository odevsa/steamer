# ADR-002: Embedded SQLite with SQLx

- **Status**: Accepted
- **Date**: 2026-11-25

## Context
A self-hosted price tracker for UmbrelOS should not require an external PostgreSQL or Redis service.

## Decision
Use embedded SQLite via `sqlx` (`sqlite` feature, WAL mode enabled, foreign keys enforced) with versioned SQL migrations in `backend/migrations/`.

## Consequences
- Zero external database dependencies; single-volume backup (`${APP_DATA_DIR}/data`).
- Compile-time checked / parameterized queries prevent SQL injection.
- Monetary values are stored as integer minor units (`INTEGER`) to avoid floating-point errors.
