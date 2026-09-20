#!/usr/bin/env bash

# 指定されたディレクトリのseed SQLをファイル名順にPostgreSQLへ適用する。
set -euo pipefail

seed_kind="${1:?production または development を指定してください}"
case "$seed_kind" in
  production|development) ;;
  *) echo "不明なseed種別: $seed_kind" >&2; exit 1 ;;
esac

seed_dir="$(cd "$(dirname "$0")/../seeds/$seed_kind" && pwd)"
shopt -s nullglob
seed_files=("$seed_dir"/*.sql)

if ((${#seed_files[@]} == 0)); then
  echo "$seed_kind seed SQLはまだありません"
  exit 0
fi

for seed_file in "${seed_files[@]}"; do
  echo "適用中: $seed_file"
  docker compose exec -T postgres psql -X --set ON_ERROR_STOP=1 -U tameshare -d tameshare -f /dev/stdin < "$seed_file"
done
