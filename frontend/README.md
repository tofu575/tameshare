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

利用可能な全ターゲットは`make help`で確認できます。
