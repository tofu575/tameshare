#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
BACKEND_DIR="$ROOT_DIR"

echo "=== [1/3] Format ==="
(cd "$BACKEND_DIR" && cargo fmt)

echo "=== [2/3] Clippy ==="
(cd "$BACKEND_DIR" && cargo clippy -- -D warnings)

echo "=== [3/3] Test ==="
(cd "$BACKEND_DIR" && cargo test)

echo ""
echo "All checks passed."
