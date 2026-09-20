# Seed SQL

Migrationはスキーマを変更し、seedは初期コンテンツを投入します。SQLファイルを各ディレクトリに置くと、ファイル名順にDocker Composeの`tameshare` DBへ適用されます。

- `production/`: 公開する初期コンテンツ。`make seed`で適用します。対象は`practices`、`sources`、`practice_sources`のみ。`experiences`と`listing_requests`の架空データは含めません。
- `development/`: 開発専用データ。`make seed-development`は`production/`を先に適用します。

各SQLファイルは`BEGIN`と`COMMIT`で囲み、エラー時にロールバックされるようにしてください。再実行時の扱いは追加するSQLごとに決め、冪等化のためだけに複雑な処理は加えません。PracticeとSourceのIDには、既存のドメイン方針に沿ってUUID v7を指定してください。

件数や投入処理が複雑になった場合は、ドメインモデルを利用するRust製seed CLIへの移行を検討します。
