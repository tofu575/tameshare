.DEFAULT_GOAL := help

.PHONY: help install run api-generate typecheck lint format build e2e-test audit check clean

help: ## 利用可能なFrontendターゲットを表示
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / {printf "  %-14s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## package-lockどおりに依存関係を導入
	npm ci

run: ## Next.js開発サーバーを起動
	direnv exec . npm run dev

api-generate: ## Backend OpenAPIからAPI Clientを生成
	npm run api:generate

typecheck: ## TypeScriptの型を検査
	npm exec tsc -- --noEmit

lint: ## Biomeでコードを検査
	npm run lint

format: ## Biomeでコードをフォーマット
	npm run format

build: ## Next.js production buildを作成
	direnv exec . npm run build

e2e-test: ## 起動中のBackend・DBを使ってFrontend主要操作を検査
	direnv exec . npm run test:e2e

audit: ## npm依存関係の脆弱性を検査
	npm audit

check: typecheck lint build ## typecheck・lint・buildを一括実行

clean: ## Next.jsの生成物を削除
	rm -rf .next
