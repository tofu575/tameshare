#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
MOBILE_DIR="$ROOT_DIR"

echo "=== [1/4] Flutter pub get (mobile) ==="
(cd "$MOBILE_DIR" && flutter pub get)

echo "=== [2/4] Dart format ==="
(cd "$MOBILE_DIR" && dart format lib test)

echo "=== [3/4] Flutter analyze ==="
(cd "$MOBILE_DIR" && flutter analyze)

echo "=== [4/4] Flutter test ==="
(cd "$MOBILE_DIR" && flutter test)

echo ""
echo "All checks passed."
