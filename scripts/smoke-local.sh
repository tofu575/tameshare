#!/usr/bin/env bash

# 起動中のBackend APIとFrontend画面が応答し、期待する内容を返すか確認する。
set -euo pipefail

BACKEND_URL="${BACKEND_URL:-http://localhost:8080}"
FRONTEND_URL="${FRONTEND_URL:-http://localhost:3000}"

# URLを取得し、responseに期待する文字列が含まれることを検査する。
check_url() {
  local label="$1"
  local url="$2"
  local expected="$3"
  local body

  body="$(curl --fail --silent --show-error "$url")"
  if [[ "$body" != *"$expected"* ]]; then
    echo "NG: ${label} responseに '${expected}' がありません" >&2
    return 1
  fi
  echo "OK: ${label}"
}

check_url "Backend health" "${BACKEND_URL}/health" '"status":"ok"'
check_url "Practice API" "${BACKEND_URL}/v1/practices?limit=1" '"items"'
check_url "Practice page" "${FRONTEND_URL}/practices" "Practiceを探す"
check_url "Listing request page" "${FRONTEND_URL}/requests/new" "掲載してほしい情報"
