---
name: security-review
description: >-
  Security-first audit workflow for Steamer backend, frontend, and container config.
  Use before completing any feature involving secrets, outbound HTTP, SQL, or Docker.
---

# Security Review

Audit changes against `docs/06-security-architecture.md`.

## Checklist

1. **Secrets & Masking**:
   - Verify all tokens/webhook URLs are encrypted via `SecretCipher` before SQLite storage.
   - Verify `GET` / `PATCH` DTOs return masked values only.
   - Verify `tracing::*` logs never print `ChannelConfig` secrets or raw URLs with tokens.
2. **SSRF Protection**:
   - Verify any user-supplied URL passes `SsrfGuard::validate_url()` on both save and dispatch.
   - Verify outbound `reqwest` clients disable automatic redirects (`Policy::none()`).
3. **SQL & Input Validation**:
   - Verify zero string-interpolated SQL queries.
   - Verify numeric bounds (`app_id > 0`, `poll_interval_secs >= 300`, `target_price_cents >= 0`).
4. **HTTP Headers & Container**:
   - Verify CSP, `X-Content-Type-Options`, and `X-Frame-Options` headers remain active in `router.rs`.
   - Verify `docker/Dockerfile` runs as non-root user.
5. **Automated Audit**:
   - Run `cargo audit` (if installed) and `pnpm --dir frontend audit`.
