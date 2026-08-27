# Template development rules

- PresentationはUsecaseの公開APIを介して処理を呼び出す。
- UsecaseはDomain ModelとGateway interfaceにのみ依存する。
- Gatewayは外部APIとDomainの相互変換を担当する。
- Packageの公開入口は`lib/<package_name>.dart`にまとめる。
- Package名、公開barrel、path dependencyは同時に更新する。
- 変更後はformat、analyze、testを実行する。
- 詳細な規約は`docs/development/coding_rules.md`を参照する。
