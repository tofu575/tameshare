# 9/25 不具合修正と回帰検査

## 実施したこと

- 9/21の手動確認で主要操作の失敗はなかったため、そのリストからの追加不具合修正はない。
- FrontendにPlaywrightのブラウザテストを追加した。Practice一覧・詳細、Experience保存、保存後の詳細表示、本人更新、更新後の詳細と自分の体験一覧を検査する。
- `make e2e-test` を追加した。テストは起動中のBackendとseed済みDBを使い、必要に応じてFrontend開発サーバーを起動する。

## 確認方法と結果

- `make e2e-test`: 1件成功。新しいブラウザコンテキストで実際の画面を操作した。
- Backend `make check`: fmt・Clippy・workspace全テスト成功。実DBのRepository integration testも成功。
- Frontend `make check`: 型検査・Biome・production build成功。初回は新しいテストファイルの整形で失敗したが、整形後に再実行して成功した。

## 残る問題

1. E2EはDBへ匿名の確認用Experienceを1件作成する。CIでは専用DBを使い、ローカルでは実行前に対象DBを確認する必要がある。
2. CIは未設定。公開環境での主要フローと権限判定も未確認。
3. production buildと開発サーバーを同じ `.next` ディレクトリで同時実行すると、先に観測した500が再発する可能性がある。確認コマンドは順番に実行する。

## 次に最初に行うこと

9/26はCIに検査を組み込み、専用DBを使う起動・seed・運用手順を整え、第三者が手順だけで起動できるか確認する。
