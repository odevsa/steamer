set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

check: check-backend check-frontend

assets:
    ./assets/generate-assets.sh

check-backend:
    cargo fmt --manifest-path backend/Cargo.toml --check
    cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings
    cargo test --manifest-path backend/Cargo.toml

check-frontend:
    @if command -v pnpm >/dev/null 2>&1; then \
        pnpm --dir frontend check && pnpm --dir frontend lint; \
    else \
        npm --prefix frontend run check && npm --prefix frontend run lint; \
    fi

dev:
    #!/usr/bin/env bash
    set -euo pipefail
    pids=()
    cleanup() {
        for pid in "${pids[@]:-}"; do
            kill "$pid" 2>/dev/null || true
        done
        wait 2>/dev/null || true
    }
    trap cleanup EXIT INT TERM
    cargo run --manifest-path backend/Cargo.toml &
    pids+=($!)
    if command -v pnpm >/dev/null 2>&1; then
        pnpm --dir frontend dev &
    else
        npm --prefix frontend run dev &
    fi
    pids+=($!)
    wait -n

dev-backend:
    cargo run --manifest-path backend/Cargo.toml

dev-frontend:
    @if command -v pnpm >/dev/null 2>&1; then \
        pnpm --dir frontend dev; \
    else \
        npm --prefix frontend run dev; \
    fi

version ver:
    sed -i -E 's/^version = "[^"]+"/version = "{{ver}}"/' backend/Cargo.toml
    sed -i -E 's/^version: "[^"]+"/version: "{{ver}}"/' umbrel/umbrel-app.yml
    sed -i -E 's|(image: odevsa/steamer:)[^[:space:]]+|\1{{ver}}|' umbrel/docker-compose.yml
    node -e 'const fs = require("fs"); const p = "frontend/package.json"; const pkg = JSON.parse(fs.readFileSync(p, "utf8")); pkg.version = "{{ver}}"; fs.writeFileSync(p, JSON.stringify(pkg, null, 2) + "\n");'
