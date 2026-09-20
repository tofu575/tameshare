# 9/24 Experience重複防止

## 実施したこと

- 既存の通常DBとテストDBを集計し、どちらも `(user_id, practice_id)` の重複グループが0件と確認してからmigrationを適用した。
- `experiences` に `(user_id, practice_id)` の一意制約を追加した。既存データに重複がある環境ではmigrationを失敗させ、データを自動削除しない。
- CommandGatewayの保存結果を実際のExperienceと新規作成フラグに変更した。PostgreSQLでは `INSERT ... ON CONFLICT DO UPDATE ... RETURNING` を使用し、作成なら201、既存行の更新なら200と同じExperience IDを返す。
- OpenAPIとFrontendの生成Clientを更新し、実DBテストに連続保存と並行保存を追加した。

## 確認方法と結果

- `make migrate` 成功。`make integration-test` 成功。Domain UseCaseとHTTP handlerのテストも成功。
- ローカル公開APIへ同じ署名付きセッションから並行POSTを2件送信し、201と200、同一Experience IDを確認した。続けてPOSTすると200で同じIDを返した。SQLで対象の組は1件、noteは最後に保存した内容と確認した。
- Chromeで既存Experienceを更新し、詳細画面の「自分のExperience」と「みんなのExperience」に更新内容を確認した。件数は増えなかった。

## 残る問題

1. 既存データに重複がある別環境ではmigration前に重複を調べ、採用するレコードと他レコードの扱いを決める必要がある。自動削除は行わない。
2. 公開環境では未確認。Frontendの主要操作を検知する自動回帰検査は9/25の対象。

## 次に最初に行うこと

9/25は主要フローのFrontend自動検査を追加し、Backend workspace全体のテストを実行する。
