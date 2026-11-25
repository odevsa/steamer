#!/usr/bin/env bash
set -euo pipefail

PAYLOAD=$(cat || true)
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if echo "$PAYLOAD" | grep -q 'frontend/src/.*\.\(svelte\|ts\)'; then
  if [ -f "$ROOT_DIR/frontend/package.json" ] && [ -d "$ROOT_DIR/frontend/node_modules" ]; then
    pnpm --dir "$ROOT_DIR/frontend" check >/dev/null 2>&1 || {
      echo '{"continue":true,"systemMessage":"Reminder: frontend type errors detected. Run pnpm --dir frontend check."}'
      exit 0
    }
  fi
fi

echo '{}'
