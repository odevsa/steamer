# ADR-001: Single-Container Rust + Static SvelteKit Architecture

- **Status**: Accepted
- **Date**: 2026-11-25

## Context
Steamer targets UmbrelOS and self-hosted Docker environments where simplicity, low memory footprint, and multi-arch (`amd64` + `arm64`) support are critical.

## Decision
Compile the SvelteKit frontend using `@sveltejs/adapter-static` and serve the built static assets directly from the Rust Axum HTTP server inside a single Docker container.

## Consequences
- Only one port (`8080`) and one container to manage in UmbrelOS.
- No Node.js runtime required in the production image.
- API routes (`/api/v1/*`) and SPA fallback coexist cleanly in Axum.
