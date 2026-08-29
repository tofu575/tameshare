# コーディング・設計ルール

更新日: 2026-08-27

## 目的

会話やレビューで合意した設計判断を、実装者ごとの記憶に依存させないための資料。
エージェントが必ず参照する要約はルートの`AGENTS.md`に置き、この文書では理由と
具体例を補足する。

## レイヤー間の入力

基本の流れは次のとおり。

```text
Presentation Form
  ↓ Presentation Mapper
Value Objectを含むHogeInput
  ↓ Interactor
Domain Model
  ↓ Gateway Mapper
永続化・API DTO
```

Presentationは入力文字列をそのままInteractorへ渡さず、入力境界でValue Objectへ
変換する。これにより、Interactor単体で見ても検証済みの値だけを受け取れる。

```dart
UpdatePhotoGroupInfoInput _mapPhotoGroupInfoInput({
  required Album album,
  required PhotoGroup photoGroup,
  required String title,
}) => UpdatePhotoGroupInfoInput(
  albumId: album.id,
  photoGroupId: photoGroup.id,
  photoGroupTitle: PhotoGroupTitle(title),
);
```

入力型は`UpdatePhotoGroupInfoInput`のように`HogeInput`で統一し、Interactorメソッドと
同じファイルに置く。入力型を探すために別ディレクトリを往復させないためである。

## InteractorとProvider

Interactorはアプリケーションの操作単位を提供する。PresentationはInteractorを
直接組み立てず、DIされた`interactorProvider`から取得する。

PresentationのDIと非同期状態管理にはRiverpodを標準として使う。Interactor、Gateway由来の
Stream、画面間で共有するDomain状態はProvider経由で注入・購読し、Widget内で独自のService
Locatorやグローバル変数を作らない。非同期の読込・更新状態は`AsyncNotifier`や
`AsyncValue`で表現し、loading・error・dataを区別する。

Flutter Hooksは、Stream購読、Controller、FocusNode、AnimationControllerなど、Widgetの
ライフサイクルに従うPresentationリソースへ使う。`useEffect()`で購読を開始した場合は必ず
cleanupで解除する。Domain状態や業務判断をHookローカル状態へ閉じ込めず、Interactorと
Riverpod Notifierへ置く。

Usecase packageの`src/`直下は、外部境界を表す`gateway/`と、アプリケーションの操作を表す
`interactor/`へ分ける。Gateway interfaceを機能別Interactorフォルダへ混在させない。

```text
usecase/lib/src/
  gateway/
    alarm_session_gateway.dart
    alarm_platform_gateway.dart
    motion_sensor_gateway.dart
  interactor/
    alarm_interactor.dart
    alarm_interactor/
      load_or_start.part.dart
      operate.part.dart
      apply_motion_reading.part.dart
```

Interactor本体のファイルにはconstructor、依存、part側へ委譲する薄い公開メソッドだけを置く。
操作の実処理は`part`で分割し、1つのpartファイルには1つの実装関数だけを置く。ファイル名は
公開メソッド名をsnake_caseへ変換した名前とする。Dartではclass本体をpartへ分割できないため、
part側はInteractorを受け取るprivateなトップレベル関数として実装する。

```text
loadOrStart()          → load_or_start.part.dart
applyMotionReading()   → apply_motion_reading.part.dart
watchMotion()          → watch_motion.part.dart
```

操作が1つだけのInteractorでも同じ形を使い、操作追加時に本体ファイルが肥大化しない構成を
維持する。関連するException型は例外として同一ファイルへまとめてよい。

Interactorは機能ごとにクラスを分ける。現在の構成例は次のとおり。

```text
interactor/
  alarm_interactor.dart
  alarm_scheduler_interactor.dart
  motion_sensor_interactor.dart
```

GatewayとInteractorの公開メソッドには、呼び出し側が実装を読まずに目的を判断できる
短いドキュメントコメントを付ける。処理手順ではなく、取得する情報、変更する状態、原本へ
影響するかなどの責務を書く。

### BackendのCommand / Query Gateway

Backendの永続化PortはAggregateごとのRepository interfaceに分割しない。Usecase側では、
状態を変更する`CommandGateway`と、状態を参照する`QueryGateway`の2つに分ける。新しい
Aggregateや操作を追加する場合も`PracticeRepository`や`ExperienceRepository`のような
Aggregate単位のinterfaceを増やさず、処理の性質に応じてどちらかのGatewayへ追加する。

Infrastructure側では、`PostgresqlGateway`がCommand / Query両方のinterfaceを実装する。
CommandとQueryの実装ファイルは責務ごとに分けてよいが、connection pool、DieselのRow変換、
schema、error mappingは同じGateway内で共有する。Usecase側のinterface名や型へPostgreSQL、
Dieselなどの永続化技術を露出させない。

```text
Interactor
  ├─ CommandGateway ─┐
  └─ QueryGateway ───┴─ PostgresqlGateway ─ Diesel ─ PostgreSQL
```

### Backend API契約

Backendの現在のHTTP API契約は
`backend/docs/openapi/tameshare.openapi.yaml`を正とする。path、HTTP method、status code、
認証要否、request / response field、validationを変更する場合は、HTTP実装とテストに加えて
OpenAPIも同じ変更で更新する。実装だけ、またはOpenAPIだけを先行させた不一致を残さない。

OpenAPIは3.1形式を使い、operationには一意な`operationId`と機能別のtagを付ける。共通の
ID、日時、URL、path / query parameter、error responseは`components`へ定義して参照する。
object schemaではrequired fieldと未知fieldの許可・禁止を明示し、主要なrequest / responseに
exampleと目的が分かるdescriptionを付ける。

JSON fieldは現在のBackend実装と同じsnake_caseを使用する。認証が必要なoperationは既存の
匿名認証によるBearer tokenを使い、User IDをrequest bodyから受け取らない。公開operationは
OpenAPI上で`security: []`を明示する。

一方で、単にInteractorメソッドをそのまま呼び出すだけの専用Providerは増やさない。
Providerを作るのは、非同期状態、画面間共有、再読込、依存Providerのinvalidateなど、
Presentation固有の状態管理がある場合である。

## PageとWidgetの構成

Page固有の部品が他画面の部品と混ざらないよう、Page単位のフォルダを使う。

```text
pages/
  album_detail/
    album_detail_page.dart
    components/
      album_info_card.dart
      photo_group_card.dart
      album_regroup_dialog.dart
```

Pageはデータ購読とイベント配線を中心にし、表示のまとまりはWidgetへ分割する。
Form入力をDomain型へ変換する関数はWidgetクラス外へ置き、buildメソッドから
バリデーションと変換の詳細を追い出す。

Dialogの結果から画面遷移、Provider更新、`setState()`へ進む場合は、共通の
`showSettledDialog()`を使う。通常の`showDialog()`が返す結果確定時点ではなく、
退出アニメーションとOverlay除去の完了後に後続処理を始めるためである。

`TextEditingController`、`FocusNode`などのUIリソースは、それを利用するStateが所有し、
Stateの`dispose()`で破棄する。Dialogを呼び出す関数で生成・破棄すると、Dialogの退出中に
リソースだけが先に破棄される可能性がある。

一度しか使わないという理由だけでクラスを作らない。関数、ローカル変数、Dartの
Recordで十分ならそれを使う。ただし、不変条件を持つ入力やDomain概念は、利用回数に
かかわらず型として表現する。

## Albumの責務

Album周辺の詳細は`docs/architecture/album_domain_model.md`を参照する。

- `Photo`の全体集合は`Album`だけが所有する。
- `PhotoGroup`は1枚以上のPhotoIdを持つ。
- `PhotoGroups`は所属の重複禁止、移動、解除、空グループの除去をAtomicに行う。
- 未整理写真は専用フィールドに保存せず、全写真から所属済み写真を除いて求める。
- 空の`Photos`を持つAlbumは作れない。

通常操作で起こり得る「最後の写真を外したのでグループが空になる」は例外制御に
しない。`PhotoGroups`が同じ操作の結果から空グループを除外する。

## 名前は概念に合わせる

同じ対象物であることを保証しない写真のまとまりを`Subject`とは呼ばない。
時間・位置から推論され、ユーザーが修正できるまとまりは`PhotoGroup`と呼び、UIでは
「グループ」と表示する。

表示だけを別名にするとMapperや変数名で翻訳が増えるため、概念が変わった場合は
Domain、Usecase、Gateway、Presentation、テスト、ドキュメントをまとめて改名する。

## Domain Modelのファイル分割

Domain Modelは原則として1ファイルに1クラスを配置する。型の責務と変更差分を小さく保ち、
目的の型をファイル名から直接探せるようにするためである。sealed階層を同一libraryへ置く必要が
ある場合は、classごとの`part`ファイルへ分ける。

同じ検証処理から返されるFailure型の集合は例外として同じファイルへ置いてよい。Failureを
利用箇所ごとに探し回るより、検証結果の閉じた集合として一覧できることを優先する。

### Rust Domain型の配置と定型実装

RustのDomain型が持つ文字数、件数、byte数、precisionなどの制約値は、module直下の
`const`へ分散させず、その型の`impl`ブロックへassociated constとして置く。検証処理、
Usecase、DTO mapper、テストが同じ定義を参照でき、制約の所有者が型自身であることを
明確にするためである。

```rust
impl DisplayName {
    pub const MIN_CHARS: usize = 1;
    pub const MAX_CHARS: usize = 50;
}
```

`Uuid`を1つだけ内包するID型は、`macros::TypedUuid`をderiveする。`generate()`、
`from_uuid()`、`as_uuid()`、`TryFrom<String>`、`Display`をID型ごとに手書きしない。
`Debug`、`Clone`、`Eq`、`Hash`、`Serialize`など、そのIDをどう扱うかに関わるtraitは
各型側で明示的にderiveする。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TypedUuid)]
pub struct ObservationId(Uuid);
```

`String`を1つだけ内包し、文字数の上下限だけで検証できる値オブジェクトは、
`macros::TypedString`をderiveする。`#[typed_string(min = ..., max = ...)]`で文字数制約を
宣言し、前後の空白を値として保持しない型には`trim`を付ける。これにより
`MIN_CHARS`、`MAX_CHARS`、`as_str()`、`TryFrom<String>`、`Display`、型ごとのErrorが
生成される。文字数はbyte数ではなくUnicode scalar valueの数として検証する。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedString)]
#[typed_string(min = 1, max = 50, trim)]
pub struct DisplayName(String);
```

文字数以外の形式検証を持つ型は`TypedString`へ寄せない。たとえばJWTのsegment数も
検証する`AccessToken`は型固有の`TryFrom<String>`を実装する。

Tokenなど有効期限を伴う値をUsecaseやGatewayの境界で扱う場合、値と期限を別々のfieldへ
分散させず、`IssuedAccessToken`や`IssuedRefreshToken`のような発行済み値オブジェクトへ
まとめる。期限の単位と命名は共通の`Timestamp`と`expire_at`へ揃える。

フォルダ名と中心となるDomain型の名前が同じ場合、中心型は`mod.rs`へ直接置いてよい。
`user/mod.rs`から`user/user.rs`の`User`を再公開するだけの一段深いmoduleは作らない。
値オブジェクトや補助Entityは従来どおり型ごとのファイルへ分ける。

derive macroへ寄せるのは、複数の型で意味と実装が完全に同じ機械的な処理だけとする。
検証条件や業務上の振る舞いが型ごとに異なる場合はmacroへ隠さず、各Domain型の`impl`へ置く。

## DTOと保存形式

このプロジェクトでは、明示的な要件がない限り古い実装や旧保存形式との後方互換性を
維持しない。Domain上の概念を改名した場合は、DTOとJSONキーも現在の概念へ揃える。

```dart
required List<PhotoGroupDto> photoGroups,
```

旧JSONキーを読むための`@JsonKey`、新旧両形式を読むdual-read、migration、互換shimは
実装者判断で追加しない。互換性が必要になった場合だけ、対象versionと移行方針を確認して
実装する。

非互換な旧データを読めなかった場合は、通常のDTO復元エラーとしてPresentationまで
伝播させる。空データへ変換したり、現在の保存値を自動上書きしたりしない。

Freezed DTO変更後は`make codegen`を実行し、現在の保存形式の往復テストを更新する。

## エラー処理とフォールバック

想定外エラーは、GatewayからUsecase、Presentationへ原則として伝播させる。アプリを
継続させるために空Listや既定値へ置き換えると、上位層は「正常に0件だった」のか
「読込に失敗した」のかを区別できない。

```dart
// NG: 読込失敗を正常な空一覧に見せ、元データも破棄する。
try {
  return decodeAlbums(raw);
} catch (_) {
  await storage.saveString(key, '[]');
  return const <Album>[];
}
```

```dart
// 基本方針: 文脈を付けても、元エラーとstack traceを維持して伝播する。
try {
  return decodeAlbums(raw);
} catch (error, stackTrace) {
  Error.throwWithStackTrace(
    AlbumRepositoryReadException(cause: error),
    stackTrace,
  );
}
```

Presentationはエラー伝播の終端として、メッセージ表示や再試行導線へ変換してよい。
ただし、エラー状態を空データや成功状態へ変えない。

### フォールバックを入れる前の確認

フォールバックが妥当な場合もある。たとえば、Photo Libraryから削除された画像を
プレースホルダーにして、残りのAlbumを閲覧可能にする挙動である。ただし、新しい
フォールバックを実装者判断で追加しない。次を提示し、事前にユーザーの確認を取る。

- 何が失敗したときに発動するか
- エラーを伝播させた場合に何が起こるか
- 代わりにどの値・表示・処理を使うか
- 元データ、再試行、ユーザー操作へどのような影響があるか
- フォールバックが発動した事実をユーザーが認識できるか

初回起動で設定値がない、検索結果が0件、指定IDが見つからない、といった仕様上の
「存在しない」は想定外エラーではない。nullable型、空コレクション、Result型などで
明示し、破損やI/O失敗と同じ`catch`へ混ぜない。

永続化APIが`bool`などで成否を返す場合は戻り値を検査する。例外が出なかったことだけで
保存成功とみなさない。

## コメントとドキュメント

コメントはコードを逐語的に説明するのではなく、次を短く残す。

- なぜ空を禁止・許可するのか
- どのクラスが整合性を保証するのか
- 状態を保持せず導出している理由
- 現在の保存形式を選んだ理由と、意図した非互換変更

Domainの構造や責務が変わった場合は、実装だけでなくクラス図も同時に更新する。

### PresentationとWidgetのコメント

PresentationのWidgetは、宣言直前に日本語で「何を表示し、どの操作を受け持つか」を
最大3行で記載する。公開・非公開を問わず、画面、カード、Dialog、状態表示など、Widgetごとの
役割がファイルを開いた時点で分かるようにする。

引数を持つ公開Widgetや関数は、ドキュメントコメント内で`[album]`、`[onSave]`のように
各引数の意味を記載する。非公開Widgetでも、引数の用途が名前だけでは判断しにくい場合は同様に
補足する。ヘルパー関数は役割を最大2行で記載し、単純な変数や分岐を逐語的には説明しない。

次の前提がある箇所は、宣言コメントとは別に利用箇所の近くへ理由を残す。

- 例外が想定される場合は、入力不正、参照切れ、永続化失敗など発生条件を記載する。
- `!`を使う場合は、直前の分岐、Domainの不変条件、Frameworkの契約など、安全と判断できる根拠を記載する。
- `build()`内の`ref.read`、非同期処理後の`mounted`確認、Controllerの所有など、Flutter固有の作法を意図的に適用している場合は、競合や再購読を避ける理由を記載する。
- Hero、Overlay、独自Route、ジェスチャー競合制御など凝った表示・演出は、初出箇所で仕組みと目的を記載する。

コメント量を増やすこと自体を目的にせず、`if`や`for`の意味、変数への代入など、コードを読めば
明らかな内容はコメントしない。

## テストの配置

テストは、検証対象を所有するDart packageの直下へ置く。`model`の単体テストをアプリルートから
実行する形にはせず、`lib/domain/model/test/`のようにpackageの`pubspec.yaml`と同じ階層の
`test/`へ配置する。package内部で階層を分ける場合は、原則として`lib/`以下の業務コンテキストに
対応させる。

```text
lib/domain/model/
  lib/src/album/...
  test/domain_model_test.dart
lib/presentation/
  lib/src/pages/album_detail/...
  test/src/pages/album_detail/album_detail_page_test.dart
test/
  wire/interactor_factory_test.dart
integration_test/
  ... 実機・エミュレータで行うアプリ全体のテスト
```

ルートの`test/`はアプリ全体のDIやpackage間の配線など、単一packageに属さない検証だけに使う。
Flutterの`testWidgets()`で画面や操作を検証していても、テスト用Widget環境で完結するものは
Widgetテストであり、実機で動かす`integration_test/`へは移さない。
