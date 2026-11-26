# ADR-001: Single-Container & Embedded Frontend Architecture

- **Status**: Accepted
- **Date**: 2026-11-25

## Context
Steamer targets UmbrelOS and self-hosted Docker environments where simplicity, single-binary distribution, low memory footprint, and multi-arch (`amd64` + `arm64`) support are critical.

## Decision
Compile the SvelteKit frontend using `@sveltejs/adapter-static` and embed the built static assets directly into the compiled Rust binary via `rust-embed`, served by the Axum HTTP server inside a single Docker container.

## Consequences
- Single self-contained Rust binary containing both backend API and frontend SPA.
- Only one port (`8080`) and one container to manage in Docker Compose or UmbrelOS.
- No Node.js runtime or external frontend asset volume required in the production image.
- API routes (`/api/v1/*`) and SPA fallback coexist cleanly in Axum.
