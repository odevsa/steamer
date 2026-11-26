# 05 — Docker, UmbrelOS & CI/CD Specification

## 1. Multi-Stage Dockerfile (`docker/Dockerfile`)

- **Stage 1 (`frontend-builder`)**: `node:22-alpine` + `pnpm`, installs frontend dependencies and builds static SPA (`frontend/build`).
- **Stage 2 (`backend-builder`)**: `rust:1-bookworm`, copies `frontend/build` and compiles `backend/` in `--release` mode embedding the static SPA into the binary via `rust-embed`.
- **Stage 3 (`runtime`)**: `debian:bookworm-slim` (with `ca-certificates` and `curl` for TLS and healthcheck), non-root user `steamer` (`UID 1000`), copies only the single compiled `steamer` binary.
- **Data Volume**: `/app/data` owned by `steamer` for SQLite DB (`steamer.db`) and auto-generated master key (`master.key`).
- **Healthcheck**: `curl` or lightweight binary flag / TCP check on `http://127.0.0.1:8080/api/v1/health`.

## 2. Task Runner (`justfile`)

Matches CI expectations (`just version <VERSION>`):
- `just version <ver>`: Updates `backend/Cargo.toml` and `frontend/package.json` versions cleanly.
- `just check`: Runs `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `pnpm --dir frontend check`, and `pnpm --dir frontend lint`.
- `just dev-backend` / `just dev-frontend`: Local development servers.

## 3. GitHub Actions (`.github/workflows/release-docker.yml`)

Modeled after `odevsa/monolai`:
- Triggers on `release: [published]` and `workflow_dispatch` (optional `version` input).
- Matrix build on `linux/amd64` (`ubuntu-latest`) and `linux/arm64` (`ubuntu-24.04-arm`).
- Uses `extractions/setup-just@v2` and runs `just version`.
- Builds and pushes per-arch digests with `docker/build-push-action@v6` to `odevsa/steamer` (overridable via `secrets.DOCKERHUB_REPOSITORY`).
- `merge` job downloads digests and runs `docker buildx imagetools create` to publish unified multi-arch manifest (`<version>`, `<major>.<minor>`, `latest`).
- Additional `.github/workflows/ci.yml` on PRs/pushes to `main` running `just check` and `cargo audit`.

## 4. UmbrelOS Packaging (`umbrel/`)

- `umbrel/umbrel-app.yml`:
  - `manifestVersion: 1`
  - `id: steamer`, `name: Steamer`, `category: utilities` / `gaming`
  - `port: 8080`, `path: ""`
  - `defaultUsername: ""`, `defaultPassword: ""` (relies on UmbrelOS auth)
- `umbrel/docker-compose.yml`:
  - Maps `${APP_DATA_DIR}/data:/app/data`
  - Exposes `APP_HOST: 0.0.0.0`, `APP_PORT: 8080`
  - `restart: on-failure`
