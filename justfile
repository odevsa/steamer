set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

check: check-backend check-frontend

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
    node -e 'const fs = require("fs"); const p = "frontend/package.json"; const pkg = JSON.parse(fs.readFileSync(p, "utf8")); pkg.version = "{{ver}}"; fs.writeFileSync(p, JSON.stringify(pkg, null, 2) + "\n");'
