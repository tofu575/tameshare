# ローカル起動・運用・公開確認手順

## 前提

Docker Compose、direnv、Rust/Cargo、Diesel CLI、Node.js 24、npmが必要。RustのPostgreSQLリンクにはlibpqが必要。リポジトリの `.envrc` に実際の値を置き、設定値や署名鍵はコミットしない。

## クリーンな環境から起動

1. リポジトリルートで `make setup` を実行する。`backend/.envrc.template` と `frontend/.envrc.template` から環境ファイルが作られ、Cargo・npmの依存関係とDiesel CLIが準備される。
2. `backend/.envrc` の空の `ANONYMOUS_AUTH_SECRET` を、`openssl rand -hex 32` などで生成した32バイト以上のランダム値に設定する。`direnv allow backend` を実行する。`DATABASE_URL` と `TEST_DATABASE_URL`、`frontend/.envrc` の `BACKEND_API_URL` が接続先と合っているか確認する。値はログへ出さない。
3. `make up` でPostgreSQL起動とmigrationを実施し、`make seed` で公開用Practice・Sourceを投入する。`make seed` は再実行できる。既存DBに重複Experienceがある場合、一意制約migrationは失敗するので、下記の事前確認を行う。
4. 別ターミナルで `make backend-run`、さらに別ターミナルで `make frontend-run` を実行する。`curl http://localhost:8080/health`、`make smoke`、`http://localhost:3000/practices` を確認する。
5. ブラウザでPractice一覧・詳細からExperienceを保存し、詳細と「自分の体験」に表示されること、編集後に内容が更新されることを確認する。`make -C frontend e2e-test` でも検査できるが、対象DBに確認用Experienceを1件作る。

Backendで必須なのは `DATABASE_URL` と `ANONYMOUS_AUTH_SECRET`。`TEST_DATABASE_URL` は実DB integration testに必須で、通常DBと分けてDB名を `_test` で終わらせる。Frontendでは `BACKEND_API_URL` が必須。`RUST_LOG` はログレベル設定。ローカルComposeはPostgreSQLをホストの54329番、Backendは8080番、Frontendは標準で3000番に公開する。

## migrationと初期データの運用

制約追加前に、対象DBで次のSQLを確認する。0件でなければmigrationを進めず、どのExperienceを残すかを個別に決めてバックアップを取る。自動削除手順は用意していない。

```sql
SELECT user_id, practice_id, count(*)
FROM experiences
GROUP BY user_id, practice_id
HAVING count(*) > 1;
```

Migrationは `make migrate`、seedは `make seed`。通常DBとテストDBを混同しない。テストは `make -C backend check`、`make -C frontend check`、起動済みのBackendとseed済みDBに対して `make -C frontend e2e-test`。Frontendのproduction buildと開発サーバーは同じ `.next` を共有するため、同時実行しない。

## 匿名セッションの運用

Backendが `POST /v1/anonymous-sessions` で署名付きtokenを発行し、FrontendはHTTP-only Cookieに保持する。署名鍵を変更・喪失すると発行済みtokenが無効になる。鍵を永続的かつ安全に保存し、すべてのBackendインスタンスで同じ値を使う。旧方式のUUID Cookieは移行対象外で、後方互換は設けない。

## 公開判断と確認

公開先と秘密鍵保管方法が決まったら、公開切替の直前に操作内容、公開先、ローカルと実環境の確認結果、残る問題を提示して判断を求める。切替後は実環境でPractice一覧・詳細、Experienceの作成・表示・本人更新、別ユーザーからの更新拒否とDB不変を確認する。公開環境で実行していない項目は完了扱いにしない。
