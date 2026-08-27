# domain

- ドメインモデルやユースケースを配置
- 例: 記録エンティティ、バリューオブジェクト、ユースケースクラス
- Interactorを配置し、Presentation層の振る舞いを集約する
- Presentation層はUsecase/Gatewayを直接組み立てず、Interactorを呼び出す
- 単純委譲のUsecaseクラスは増やさず、必要な振る舞いはInteractorに集約する
- Interactorの操作は`album/`、`album_read/`、`auto_group/`、`photo_library/`の
  コンテキスト単位で配置する
- Album Aggregate内の概念であるPhotoGroup操作は`album/`へ置く
