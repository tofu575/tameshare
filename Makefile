.DEFAULT_GOAL := help

.PHONY: help setup env install postgres-up postgres-down db-migrate backend-run frontend-run api-generate smoke check

help: ## 利用可能なRootターゲットを表示
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / {printf "  %-16s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

setup: env install ## ローカル環境ファイル作成と依存関係導入を行う

env: ## 各レイヤーの.envrcをtemplateから作成して許可する
	@test -f backend/.envrc || cp backend/.envrc.template backend/.envrc
	@test -f frontend/.envrc || cp frontend/.envrc.template frontend/.envrc
	direnv allow backend
	direnv allow frontend

install: ## 各レイヤーの依存関係を導入する
	$(MAKE) -C backend install
	$(MAKE) -C frontend install

up: ## PostgreSQLを起動し、healthcheck後にmigrationを適用する
	$(MAKE) -C backend db-up
	$(MAKE) -C backend db-migrate

down: ## ローカルPostgreSQLを停止する
	$(MAKE) -C backend db-down

migrate: ## BackendのDB migrationを適用する
	$(MAKE) -C backend db-migrate

backend-run: ## Cargo runでBackend APIを起動する
	$(MAKE) -C backend run

frontend-run: ## Next.js開発サーバーを起動する
	$(MAKE) -C frontend run

api-generate: ## OpenAPIからFrontend API Clientを再生成する
	$(MAKE) -C frontend api-generate

smoke: ## 起動中のBackend・Frontendの主要URLを疎通確認する
	./scripts/smoke-local.sh

check: ## Backend・Frontendの全検査を実行する
	$(MAKE) -C backend check
	$(MAKE) -C frontend check
