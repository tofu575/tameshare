.DEFAULT_GOAL := help

.PHONY: help install run build release test integration-test fmt fmt-check clippy check clean db-up db-down db-migrate seed-production seed-development up down

help: ## 利用可能なターゲットを表示
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / {printf "  %-12s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Cargo依存関係とDiesel CLIを導入
	cargo fetch --locked
	@if ! command -v diesel >/dev/null; then \
		libpq_dir="$${PQ_LIB_DIR:-}"; \
		if [ -z "$$libpq_dir" ] && [ -d /opt/homebrew/opt/libpq/lib ]; then libpq_dir=/opt/homebrew/opt/libpq/lib; fi; \
		if [ -z "$$libpq_dir" ] && [ -d /usr/local/opt/libpq/lib ]; then libpq_dir=/usr/local/opt/libpq/lib; fi; \
		if [ -n "$$libpq_dir" ]; then \
			PQ_LIB_DIR="$$libpq_dir" RUSTFLAGS="$${RUSTFLAGS:-} -L native=$$libpq_dir" cargo install diesel_cli --version 2.2.12 --no-default-features --features postgres --locked; \
		else \
			cargo install diesel_cli --version 2.2.12 --no-default-features --features postgres --locked; \
		fi; \
	fi

run: ## Actix Web サーバーを起動
	direnv exec . cargo run --bin api

build: ## workspace 全体をビルド
	cargo build --workspace

release: ## workspace 全体をリリースビルド
	cargo build --workspace --release

test: ## workspace 全体のテストを実行
	direnv exec . cargo test --workspace

integration-test: ## PostgreSQL Repository integration testを実行
	direnv exec . cargo test -p postgres_gateway --test repository_integration

db-up: ## ローカルPostgreSQLを起動してhealthcheckを待つ
	docker compose up -d --wait postgres

db-down: ## ローカルPostgreSQLを停止
	docker compose down

db-migrate: ## Diesel CLIでmigrationを適用
	@command -v diesel >/dev/null || { echo "diesel CLIが必要です: cargo install diesel_cli --no-default-features --features postgres"; exit 1; }
	direnv exec . env RUST_LOG=warn diesel migration run

seed-production: ## DATABASE_URLへ本番用seed SQLを適用
	direnv exec . ./scripts/run-seeds.sh production

seed-development: seed-production ## DATABASE_URLへ本番用・開発用seed SQLを適用
	direnv exec . ./scripts/run-seeds.sh development

up: db-up ## db-upの互換alias

down: db-down ## db-downの互換alias

fmt: ## Rust コードをフォーマット
	cargo fmt --all

fmt-check: ## フォーマット差分がないことを確認
	cargo fmt --all -- --check

clippy: ## Clippy の警告をエラーとして検査
	cargo clippy --workspace --all-targets --all-features -- -D warnings

check: fmt-check clippy test ## format・Clippy・testを一括実行

clean: ## Cargo のビルド成果物を削除
	cargo clean

assemble-debug:
	./scripts/assemble-debug.sh
