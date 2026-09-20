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
seeds/                   # 初期コンテンツを投入するSQL
```

依存の流れは `Actix handler -> Interactor -> Port <- Gateway` です。
Domain crateはDiesel、PostgreSQL、HTTPへ依存しません。

## Database

環境変数ファイルを作成して値を設定し、direnvで読み込んでからPostgreSQLを起動します。

```bash
cp .envrc.template .envrc
# 必要なら.envrcの接続先をローカル環境に合わせて編集
direnv allow
make db-up
make db-migrate
```

ローカルDBを`psql`で確認する場合は、`backend`ディレクトリで次を実行します。

```bash
docker compose exec postgres psql -U tameshare -d tameshare
```

接続後は`\dt`でテーブル一覧を表示し、`\q`で終了できます。

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

## Seed

Migration適用後に、`backend`ディレクトリから初期コンテンツSQLを実行します。起動中のDocker Composeの`postgres`コンテナ内で`psql`を実行するため、ホスト側への`psql`のインストールは不要です。接続先はComposeで設定した`tameshare` DBです。

```bash
make seed              # seeds/production/*.sql を適用
make seed-production   # seeds/production/*.sql を適用
make seed-development  # 本番用を適用後、seeds/development/*.sql を適用
```

リポジトリのルートからも同じコマンドを実行できます。`make seed`は`make seed-production`と同じ内容です。SQLの配置先と方針は[seeds/README.md](seeds/README.md)を参照してください。

## 主なRoutes

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/health` | Actix の疎通確認 |
| `GET` | `/v1/practices` | Practice一覧 |
| `GET` | `/v1/practices/{id}` | Practice詳細 |
| `GET, POST` | `/v1/practices/{id}/experiences` | Experience一覧・作成 |
| `PATCH` | `/v1/experiences/{id}` | Experience更新 |
| `POST` | `/v1/listing-requests` | 掲載リクエスト作成 |

## Run

```bash
make run
```

別ターミナルから確認できます。

```bash
curl http://localhost:8080/health
curl http://localhost:8080/v1/practices
```

## Build and test

```bash
make build
make test
make integration-test
make check
```
