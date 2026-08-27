# Actix Web clean architecture template

Actix Web のルーティングと、Clean Architecture の依存方向を示す最小テンプレートです。
具体的な Model、Usecase、外部サービス実装は含めていません。

## 構成

```text
apps/                    # Actix サーバー起動・ルーティング組み立て・Middleware
domain/model/            # Entity / Value Object の配置先（初期状態は空）
domain/usecase/          # Interactor と Gateway port
gateway/http_handlers/   # Actix handler
gateway/dummy_gateway/   # 起動確認用の唯一の Gateway 実装
libs/wire/               # 依存関係を組み立てる Composition Root
macros/                  # 汎用 derive macro の例
```

依存の流れは `Actix handler -> Interactor -> Gateway trait <- DummyGateway` です。

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
```

実アプリへ展開するときは、`domain/model` にドメイン型を追加し、
`domain/usecase` の例示用 `execute` をユースケース固有の操作へ置き換え、
`DummyGateway` の代わりとなる Gateway 実装を `libs/wire` で注入してください。
