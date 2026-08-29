# tameshare backend

短時間で試せる方法（Practice）を、出典（Source）と実際に試した体験
（Experience）へつなぐサービスのバックエンドです。

## 構成

```text
apps/                    # Actix サーバー起動・ルーティング組み立て・Middleware
domain/model/            # Entity / Value Object
domain/usecase/          # Interactor と Repository/Gateway port
gateway/http_handlers/   # Actix handler
gateway/dummy_gateway/   # 起動確認用の唯一の Gateway 実装
gateway/postgres_gateway/# Diesel + PostgreSQL Repository実装
libs/wire/               # 依存関係を組み立てる Composition Root
macros/                  # 汎用 derive macro の例
migrations/              # Diesel migrations
```

依存の流れは `Actix handler -> Interactor -> Port <- Gateway` です。
Domain crateはDiesel、PostgreSQL、HTTPへ依存しません。

## Database

環境変数ファイルを作成して値を設定し、direnvで読み込んでからPostgreSQLを起動します。

```bash
cp .envrc.template .envrc
# .envrcのDATABASE_URL、TEST_DATABASE_URL、RUST_LOGをローカル環境に合わせて編集
direnv allow
docker compose up -d --wait postgres
```

`TEST_DATABASE_URL`は通常DBと分離し、DB名が`_test`で終わる接続先を指定してください。
Composeの初回初期化では通常DBとは別に`tameshare_test`を作成します。

Diesel CLIを利用する場合、migrationの往復は次のように確認できます。

```bash
diesel migration run
diesel migration revert
diesel migration run
```

Repository integration test自身も、開始時にmigrationを全rollbackしてから再適用します。
ローカルビルドにはlibpqが必要です。Homebrewのkeg-only配置は自動検出し、
それ以外の非標準配置では`PQ_LIB_DIR`を指定できます。

## Routes

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/health` | Actix の疎通確認 |
| `GET` | `/example` | Interactor から DummyGateway を呼ぶ最小例 |

`/example` は成功時に `204 No Content` を返します。

## Run

```bash
cargo run --bin api
```

別ターミナルから確認できます。

```bash
curl http://localhost:8080/health
curl -i http://localhost:8080/example
```

## Build and test

```bash
cargo build --workspace
cargo test --workspace
cargo test -p postgres_gateway --test repository_integration
```

HTTP APIと本格的なUseCaseは次段階の対象です。現状の `/example` はテンプレートの
依存方向を保つための疎通用であり、RepositoryはまだHTTP層へ配線していません。
