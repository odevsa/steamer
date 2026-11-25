# ADR-003: Clean Architecture & Trait-Driven SOLID Boundaries

- **Status**: Accepted
- **Date**: 2026-11-25

## Context
AI-assisted development can easily degrade into monolithic handlers or tightly coupled ad-hoc code if boundaries are not enforced.

## Decision
Enforce Clean Architecture layers (`domain`, `application`, `infrastructure`, `presentation`) with async traits (`ports`) for all repositories, Steam HTTP calls, encryption, and notification channels.

## Consequences
- Use cases can be unit-tested with in-memory mock implementations of `ports`.
- Adding a new notification channel or swapping storage requires zero changes to `domain` or `application`.
