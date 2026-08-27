import 'package:model/model.dart';

abstract interface class ItemGateway {
  /// 保存されているテンプレート項目をすべて取得します。
  Future<List<TemplateItem>> findAll();
}
