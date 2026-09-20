# 試Share Frontend

Next.js App Routerで構築したFrontendです。Backend APIへのアクセスはServer側から行います。

## Setup

Rootから`make setup`を実行するか、Frontendだけを準備します。

```bash
cp .envrc.template .envrc
direnv allow
make install
```

## Run

Backendを起動してから開発サーバーを起動します。

```bash
make run
```

## Development commands

```bash
make api-generate
make typecheck
make lint
make build
make check
```

## Browser test

Backend、PostgreSQL、production seedを起動した状態で、初回だけ `npx playwright install chromium` を実行し、`make e2e-test` で主要フローを検査します。テストは新しい匿名セッションでExperienceを1件DBへ作成します。専用DBを使うCIではそのDBへseedを適用してから実行します。

利用可能な全ターゲットは`make help`で確認できます。
