# 試Share

短時間で試せるPracticeと、実際に試したExperienceを共有するサービスです。

## ローカル開発

Docker、Rust、Node.js、direnvを用意し、最初に環境ファイルと依存関係を準備します。

```bash
make setup
```

PostgreSQLを起動した後、BackendとFrontendをそれぞれ別ターミナルで起動します。

```bash
make postgres-up
make backend-run
make frontend-run
```

`make postgres-up`はPostgreSQLのhealthcheckを待ってからmigrationも適用します。
migrationだけを再適用する場合は次を実行します。

```bash
make db-migrate
```

Frontendは http://localhost:3000、Backendは http://localhost:8080 で起動します。
両方の起動後、主要な接続を確認できます。

```bash
make smoke
```

## 検査・コード生成

```bash
make check
make api-generate
```

Root Makefileは各レイヤーの入口だけを提供します。個別のbuild、lint、test、formatは
`backend/Makefile`と`frontend/Makefile`を利用してください。

```bash
make -C backend help
make -C frontend help
```
