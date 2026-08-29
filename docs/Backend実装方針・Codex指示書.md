# 今回の目的

既存のBackendテンプレートを利用して、試せる方法と体験を集約するWebサービスのBackend基盤を実装してください。

実装前に、別途用意されている以下のドキュメントを確認してください。

- サービスコンセプト・MVP方針
- ドメインモデル・制約

今回はFrontendを実装しません。

また、HTTP APIやUseCaseの本格実装も次の段階とし、まずはDomain Modelと永続化基盤を安定させることを優先します。

# 技術構成

以下を前提としてください。

- Rust
- Clean Architecture
- PostgreSQL
- Diesel
- Diesel migrations
- UUID version 7
- Docker / Docker Compose

既存BackendテンプレートのWorkspace構成、crate構成、命名規則、Error handling、Test方針を優先してください。

合理的な理由なく独自構成へ変更しないでください。

# 今回実装する範囲

## Domain Model

「ドメインモデル・制約」に基づいて必要なDomain Modelを実装してください。

少なくとも以下を対象とします。

- Practice
- Source
- Experience
- ListingRequest
- ListingRequestStatus
- 各種ID
- 必要なValue Object

Domain層からDieselやPostgreSQLなどのInfrastructure詳細を参照しないでください。

## PostgreSQL schema

Domain Modelを永続化できるschemaを設計してください。

特に以下を考慮してください。

- UUID version 7
- `TIMESTAMPTZ`
- Foreign Key
- Unique Constraint
- 必要最低限のIndex
- SourceとPracticeのN:M関係
- ListingRequestの重複防止

根拠のないIndexを大量に作らないでください。

## Diesel migration

Diesel migrationでschemaを管理してください。

`up.sql`と`down.sql`を作成し、

```text
migration適用
↓
rollback
↓
再適用
```

が成立する状態にしてください。

## Repository

Domain側でRepositoryの抽象を定義し、Infrastructure側にDiesel + PostgreSQLによる実装を追加してください。

RepositoryはDomain ModelとDatabase Modelの境界を明確にしてください。

Diesel固有の型をDomainへ漏らさないでください。

Repository APIは現在のMVPで必要になる操作から設計してください。

CRUDを機械的にすべて追加しないでください。

## Docker Compose

ローカル開発用PostgreSQLをDocker Composeで起動できるようにしてください。

Rust Backend自体をDocker内で動作させることは必須ではありません。

想定する開発環境は以下です。

```text
PostgreSQL
  → Docker Compose

Rust Backend
  → ローカルcargo
```

必要な環境変数のサンプルを用意してください。

秘密情報はコミットしないでください。

## Test

必要なDomain TestとRepository Integration Testを追加してください。

最低限、以下を確認してください。

- Domain上の主要な不変条件
- ExperienceNoteの制約
- ListingRequestの重複防止
- Repositoryへの保存・取得
- SourceとPracticeのN:M関係
- migration適用後のDBを利用したIntegration Test

テストのためだけに本番コードを不自然な設計へ変更しないでください。

# ListingRequest重複防止

同一`SourceUrl`に対するListingRequestが複数存在しないことは重要な制約です。

事前のSELECTによる存在確認だけでは、並行処理時に競合する可能性があります。

そのため、PostgreSQLのConstraint等も利用し、DBレベルでも重複を防止してください。

一方で、重要なDomain RuleがDB schemaだけを読まなければ分からない状態にもしてくださいません。

DomainとDBの双方で意図が読み取れる構造を優先してください。

# 実装しないもの

今回以下は実装対象外です。

- Frontend
- Next.js
- HTTP API
- OpenAPI
- UseCase / Interactorの本格実装
- Admin UI
- Admin API
- Recommendation
- AI関連機能
- 自動クロール
- 課金
- 高度な検索
- 高度なURL正規化

必要以上に先回りして実装しないでください。

# 実装前に確認すること

既存Backendテンプレートを確認し、少なくとも以下を把握してから作業してください。

1. Workspace / crate構成
2. 各crateの責務
3. Clean Architecture上の依存方向
4. Error handling
5. async runtime
6. Database関連の既存実装
7. Test方針

既存方針と本指示が矛盾しない場合は、既存テンプレートの方針を優先してください。

# 実装方針

## 過度に抽象化しない

Clean Architectureだからという理由だけで、意味の薄いInterface、Wrapper、Serviceを大量に作らないでください。

## 将来予測で設計を膨らませない

現時点のMVP要件から説明できないEntity、Column、Repository method、状態などを追加しないでください。

## Domain RuleとDB Constraintを使い分ける

重要な業務ルールはDomainコードから理解できるようにしてください。

同時に、並行処理によって壊れる可能性のある制約についてはDB Constraintも利用してください。

# 実装後の確認

利用可能な環境の範囲で、最低限以下を実行してください。

```text
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo test --workspace --all-features
git diff --check
```

加えて、

- Docker ComposeでPostgreSQLが起動できる
- migrationを適用できる
- migrationをrollbackできる
- Repository Integration Testが通る

ことを確認してください。

環境上の制約で実行できなかった場合は、成功したものとして扱わず、明示してください。

# 完了時の報告

実装完了後、以下をまとめてください。

1. 追加・変更したもの
2. 採用したAggregate境界とその理由
3. DB schemaの概要
4. SourceとPracticeのN:Mをどう表現したか
5. ListingRequest重複をどのように保証したか
6. ExperienceNoteの制約をどこで保証したか
7. 実行したTestと結果
8. 判断を保留した設計事項
9. 次のPRで実装すべき内容

次の段階では、このDomain / Repositoryを前提としてUseCase、OpenAPI、HTTP APIを実装する予定です。