# Mobile Multi-Package Template

Flutterアプリを複数のローカルPackageに分割するための、最小構成テンプレートです。

## 構成

```text
lib/
├─ domain/
│  ├─ model/                 # Entity / Value Object
│  └─ usecase/               # Interactor / Gateway interface
├─ gateway/
│  └─ dummy_gateway/         # 開発・テスト用のダミー実装
├─ presentation/             # Flutter UI
├─ wire/                     # DI
└─ main.dart
```

Gatewayは最初から細分化せず、ダミーの1 Packageだけを置いています。外部API、
永続化、プラットフォーム機能が必要になった時点で、責務ごとのPackageを追加してください。

## はじめ方

1. `mobile_template`、Bundle ID、表示名をプロジェクト名へ変更します。
2. `TemplateItem`を実際のDomain Modelへ置き換えます。
3. `DummyGateway`を実サービスへ置き換えるか、責務ごとのGateway Packageを追加します。
4. `TemplateApp`へ画面とProviderを追加します。

```sh
flutter pub get
flutter analyze
flutter test
```
