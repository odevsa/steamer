#!/usr/bin/env bash
set -euo pipefail

PAYLOAD=$(cat || true)
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if echo "$PAYLOAD" | grep -q 'backend/.*\.rs'; then
  if [ -f "$ROOT_DIR/backend/Cargo.toml" ]; then
    cargo fmt --manifest-path "$ROOT_DIR/backend/Cargo.toml" -- --check >/dev/null 2>&1 || {
      echo '{"continue":true,"systemMessage":"Reminder: Run cargo fmt and cargo clippy in backend/ before finishing."}'
      exit 0
    }
  fi
fi

echo '{}'
