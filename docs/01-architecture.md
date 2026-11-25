# 01 — System & Clean Architecture

## 1. Architectural Principles

Steamer strictly follows **Clean Architecture** and **SOLID** principles across both backend and frontend:

- **Dependency Rule**: Dependencies point strictly inward. `domain` depends on nothing external. `application` depends only on `domain`. `infrastructure` and `presentation` depend on `application` and `domain` via traits/interfaces.
- **Interface Segregation & Dependency Inversion**: Every external boundary (database persistence, Steam HTTP client, secret encryption, notification delivery, clock/locale detection) is abstracted behind an async Rust `trait`.
- **Small, Single-Purpose Modules**: No god-files or monolithic handlers. Each use case, entity, repository, and notifier lives in its own focused file.
- **No Ad-Hoc Shortcuts**: Cross-cutting concerns (error mapping, SSRF validation, rate limiting, secret masking) live in dedicated shared modules.

## 2. Repository Layout

```text
steamer/
├── backend/
│   ├── Cargo.toml
│   ├── migrations/                     # SQLx SQLite migrations
│   └── src/
│       ├── main.rs                     # Composition root (DI wiring only)
│       ├── config.rs                   # Env config & OS locale/region detection
│       ├── domain/
│       │   ├── mod.rs
│       │   ├── entities/               # WatchlistItem, GamePrice, NotificationChannel, AlertLog, AppSettings
│       │   ├── value_objects/          # AppId, MoneyCents, CountryCode, SecretString
│       │   ├── errors.rs               # DomainError enum
│       │   └── ports/                  # Traits: WatchlistRepo, ChannelRepo, SteamGateway, Notifier, SecretCipher
│       ├── application/
│       │   ├── mod.rs
│       │   ├── use_cases/              # search_games, add_watch_item, evaluate_prices, manage_channels, etc.
│       │   └── services/               # PriceMonitorWorker, AlertEvaluator
│       ├── infrastructure/
│       │   ├── mod.rs
│       │   ├── persistence/            # SQLx SQLite implementations of domain ports
│       │   ├── steam/                  # Reqwest Steam Store client + rate limiter
│       │   ├── notifications/          # NotifierRegistry + Discord, Telegram, ntfy, Webhook impls
│       │   └── security/               # AesGcmCipher, SsrfGuard
│       └── presentation/
│           ├── mod.rs
│           ├── router.rs               # Axum router + security headers + static SPA fallback
│           ├── handlers/               # Thin HTTP handlers delegating to use cases
│           ├── dto/                    # Request/Response structs + validation
│           └── error_mapper.rs         # Maps DomainError -> safe HTTP status & JSON error
├── frontend/
│   ├── package.json
│   ├── messages/                       # Paraglide i18n JSON files (en.json, pt-br.json)
│   └── src/
│       ├── app.css                     # Tailwind v4 + semantic theme tokens
│       ├── lib/
│       │   ├── api/                    # Typed API client & DTO types
│       │   ├── stores/                 # Theme, locale, watchlist reactive state
│       │   └── components/ui/          # Atomic Design (atoms, molecules, organisms, templates)
│       └── routes/                     # SvelteKit pages
├── docker/
│   └── Dockerfile                      # Multi-stage build (Node -> Rust -> Distroless/Debian-slim)
├── umbrel/
│   ├── umbrel-app.yml
│   └── docker-compose.yml
├── .github/workflows/
│   └── release-docker.yml
└── justfile
```

## 3. Dependency Injection & Composition Root

`backend/src/main.rs` is the **only** place that instantiates concrete infrastructure structs (`SqliteWatchlistRepo`, `SteamHttpGateway`, `AesGcmSecretCipher`, `DiscordNotifier`, etc.) and injects them as `Arc<dyn Port>` into `application` use cases and `presentation` state.

## 4. Concurrency & Background Worker

- `PriceMonitorWorker` runs as a supervised `tokio::spawn` task alongside the Axum HTTP server with graceful shutdown (`tokio::signal`).
- An `mpsc` or `Notify` trigger allows the `POST /api/watchlist/check` endpoint to request an immediate cycle while respecting a minimum cooldown (e.g., 60 seconds) to protect Steam API quotas.
