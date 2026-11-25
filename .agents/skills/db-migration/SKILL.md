---
name: db-migration
description: >-
  Safe workflow for creating and applying SQLite schema migrations with SQLx.
  Use when adding tables, columns, or indexes in backend/migrations/.
---

# Database Migration Workflow

## Steps

1. **Update Schema Spec**:
   - Document the schema change in `docs/02-data-model-and-api.md` first.
2. **Create Migration File**:
   - Create a sequential SQL file in `backend/migrations/` (e.g., `0002_add_index.sql`).
   - Never modify or delete an already-committed migration file.
   - Store monetary amounts as `INTEGER` minor units (cents), timestamps as ISO-8601 UTC `TEXT`, and booleans as `INTEGER` (`0`/`1`).
3. **Update Domain & Infrastructure**:
   - Update domain entity in `backend/src/domain/entities/`.
   - Update SQLx repository queries in `backend/src/infrastructure/persistence/` using parameterized queries only.
4. **Verify**:
   - Run `cargo test` in `backend/` to verify migrations apply cleanly on a fresh SQLite database and repository tests pass.
