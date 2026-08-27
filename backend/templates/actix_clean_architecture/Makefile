.DEFAULT_GOAL := help

.PHONY: help run build release test fmt fmt-check clippy check clean

help: ## 利用可能なターゲットを表示
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / {printf "  %-12s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

run: ## Actix Web サーバーを起動
	cargo run --bin api

build: ## workspace 全体をビルド
	cargo build --workspace

release: ## workspace 全体をリリースビルド
	cargo build --workspace --release

test: ## workspace 全体のテストを実行
	cargo test --workspace

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
