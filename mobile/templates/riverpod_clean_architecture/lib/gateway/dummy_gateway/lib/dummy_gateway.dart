import 'package:model/model.dart';
import 'package:usecase/usecase.dart';

/// メモリ上の固定データを返す、開発・テスト用のGatewayです。
final class DummyGateway implements ItemGateway {
  DummyGateway({List<TemplateItem> initialItems = const <TemplateItem>[]})
    : _items = List<TemplateItem>.from(initialItems);

  final List<TemplateItem> _items;

  @override
  Future<List<TemplateItem>> findAll() async =>
      List<TemplateItem>.unmodifiable(_items);
}
