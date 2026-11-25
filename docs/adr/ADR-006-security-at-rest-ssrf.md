# ADR-006: Encryption at Rest & SSRF Guard without MVP Auth

- **Status**: Accepted
- **Date**: 2026-11-25

## Context
UmbrelOS provides network/app-level authentication, so MVP does not include a login screen. However, stored webhook URLs and bot tokens must never be exposed.

## Decision
Encrypt all notification channel configurations at rest using AES-256-GCM (`AesGcmCipher`), mask secrets in all API responses, and validate outbound URLs with `SsrfGuard` (DNS IP range checks + disabled redirects).

## Consequences
- Compromising an API read or database dump without `master.key` does not leak bot tokens or webhooks.
- Prevents Steamer from being abused as an internal network SSRF proxy.
