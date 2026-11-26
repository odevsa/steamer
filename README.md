<div align="center">

# Steamer

**Self-hosted Steam price tracker & deal alerter built with Rust and Svelte.**

[![Release Docker](https://img.shields.io/github/actions/workflow/status/odevsa/steamer/release-docker.yml?branch=main&style=flat-square&logo=githubactions&label=build)](https://github.com/odevsa/steamer/actions/workflows/release-docker.yml)
[![Docker Hub](https://img.shields.io/docker/v/odevsa/steamer?style=flat-square&logo=docker&label=docker)](https://hub.docker.com/r/odevsa/steamer)
[![Docker Pulls](https://img.shields.io/docker/pulls/odevsa/steamer?style=flat-square&logo=docker)](https://hub.docker.com/r/odevsa/steamer)
[![Platform](https://img.shields.io/badge/platform-amd64%20%7C%20arm64-blue?style=flat-square)](https://hub.docker.com/r/odevsa/steamer)
[![License](https://img.shields.io/github/license/odevsa/steamer?style=flat-square)](LICENSE)

</div>

---

Steamer monitors Steam Store prices for games on your watchlist and sends instant alerts when a game drops to or below your target price — or whenever it goes on sale if no target price is set. Designed to run as a lightweight single container on **UmbrelOS**, Raspberry Pi, or any Docker host.

## Features

- **Flexible Price Alerts** — Trigger notifications at `price <= target` or on *any* discount when no target is set.
- **Multi-Channel Notifications** — Simultaneous support for **Discord**, **Telegram**, **ntfy**, and **Generic JSON Webhooks**.
- **Anti-Spam Deduplication** — Smart state tracking prevents duplicate alerts during the same sale window unless the price drops further.
- **Auto-Detected Region & Language** — Infers Steam store currency (fallback `USD`) and UI language (fallback `en`) from system/browser settings, both overridable in Settings.
- **Security-First** — Notification tokens and webhook URLs are encrypted at rest in SQLite and masked in API responses; built-in SSRF protection for outbound webhooks.
- **Single-Container Footprint** — Compiled Rust binary serving an embedded SvelteKit SPA with embedded SQLite.

## Tech Stack

| Layer | Technology |
| :--- | :--- |
| **Backend** | Rust, Axum, Tokio, SQLx (SQLite), Reqwest |
| **Frontend** | SvelteKit, TypeScript, Tailwind CSS, `bits-ui`, `lucide-svelte`, `paraglide-js` |
| **Container** | Multi-stage Docker (`linux/amd64`, `linux/arm64`), UmbrelOS ready |

## Quick Start

### Docker Compose

```yaml
services:
  steamer:
    image: docker.io/odevsa/steamer:latest
    container_name: steamer
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      - PORT=8080
      - HOST=0.0.0.0
      - DATA_DIR=/app/data
    volumes:
      - ./.steamer/data:/app/data
```

Or build locally with the included [docker-compose.yml](docker-compose.yml):

```bash
docker compose up -d --build
```

Open `http://localhost:8080`.

### UmbrelOS

Steamer ships with UmbrelOS app definitions under `umbrel/` (`umbrel-app.yml` and `docker-compose.yml`), persisting all SQLite data and encryption keys in `${APP_DATA_DIR}/data`.

## Configuration

| Variable | Default | Description |
| :--- | :--- | :--- |
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `8080` | HTTP server port |
| `DATA_DIR` | `./data` | Directory for SQLite database and generated encryption key |
| `STEAMER_MASTER_KEY` | *auto-generated* | Optional 32-byte hex/base64 key for encrypting stored secrets |
| `POLL_INTERVAL_SECS` | `3600` | Default background price check interval (overridable in UI) |
| `RUST_LOG` | `info` | Tracing log filter |

## Development

Prerequisites: Rust (stable), Node.js 22+, `pnpm`, and `just`.

```bash
just dev-backend
just dev-frontend
```

Run checks and tests:

```bash
just check
```

## Architecture & Documentation

Detailed specifications, Clean Architecture boundaries, design system tokens, and security guidelines live in [`docs/`](docs/):

- [Vision & PRD](docs/00-vision-and-prd.md)
- [Clean Architecture](docs/01-architecture.md)
- [Data Model & REST API](docs/02-data-model-and-api.md)
- [Steam Integration & Notifiers](docs/03-steam-and-notifications.md)
- [Frontend Design System](docs/04-frontend-design-system.md)
- [Docker, UmbrelOS & CI/CD](docs/05-docker-umbrel-ci.md)
- [Security Architecture](docs/06-security-architecture.md)

## License

MIT
