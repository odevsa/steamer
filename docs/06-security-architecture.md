# 06 — Security-First Architecture

Even though Steamer runs behind UmbrelOS without an internal login screen in MVP, **data protection and defense-in-depth are mandatory**.

## 1. Encryption at Rest (Secrets)

- **Sensitive Fields**: Discord webhook URLs, Telegram bot tokens, Telegram chat IDs, `ntfy` access tokens, and custom webhook URLs/headers.
- **Cipher**: `Aes256Gcm` (`aes-gcm` crate) with a random 96-bit nonce per encryption (`nonce || ciphertext`, base64-encoded).
- **Key Management**:
  1. If `STEAMER_MASTER_KEY` env var is provided (32 bytes hex/base64), use it.
  2. Otherwise, on first boot, generate a cryptographically random 32-byte key (`OsRng`) and persist it to `{DATA_DIR}/master.key` with `0600` file permissions.
- **API Masking**: `GET /api/v1/channels` and `PATCH /api/v1/channels/:id` **never** return plaintext secrets. Return masked values (e.g., `https://discord.com/api/webhooks/••••1234` or `••••••••abcd`).

## 2. SSRF Protection (`SsrfGuard`)

User-configured URLs (`ntfy.server_url`, `webhook.url`, `discord.webhook_url`) must pass `SsrfGuard::validate_url()` before saving AND before sending:
- Allow only `https://` (and `http://` only if `ALLOW_HTTP_WEBHOOKS=true` or for explicit `ntfy` self-hosted local instances when configured).
- Resolve DNS and reject loopback (`127.0.0.0/8`, `::1`), private RFC1918 (`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`), link-local (`169.254.0.0/16`, `fe80::/10`), and multicast ranges by default.
- Disable HTTP redirects (`reqwest::redirect::Policy::none()`) on outbound notification requests to prevent redirect-based SSRF bypasses.

## 3. Input Validation & SQL Injection Prevention

- All SQL queries use `sqlx::query!` / `sqlx::query_as!` parameterized bindings. Dynamic string concatenation in SQL is strictly forbidden.
- Validate `app_id` (`> 0`), `target_price_cents` (`>= 0` and reasonable upper bound), `poll_interval_secs` (`>= 300`), and `country_code` (2-letter uppercase ISO alpha-2).

## 4. HTTP Security Headers & Error Hygiene

- Axum middleware sets:
  - `X-Content-Type-Options: nosniff`
  - `X-Frame-Options: SAMEORIGIN`
  - `Referrer-Policy: strict-origin-when-cross-origin`
  - `Content-Security-Policy` restricting scripts/styles to `'self'` and Steam CDN images (`https://shared.akamai.steamstatic.com`, `https://cdn.akamai.steamstatic.com`, `https://store.steampowered.com`).
- `error_mapper.rs` logs full internal details via `tracing::error!` (with secrets redacted) and returns generic, safe error codes/messages to clients.

## 5. Supply Chain & Container Hardening

- Run container as non-root `steamer` user (`UID 1000`).
- Run `cargo audit` and `pnpm audit` in CI.
