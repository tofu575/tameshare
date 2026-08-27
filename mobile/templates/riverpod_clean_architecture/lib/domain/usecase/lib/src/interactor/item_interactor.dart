import 'package:model/model.dart';

import '../gateway/item_gateway.dart';

part 'item_interactor/list_items.part.dart';

final class ItemInteractor {
  const ItemInteractor({required ItemGateway itemGateway})
    : _itemGateway = itemGateway;

  final ItemGateway _itemGateway;

  /// 一覧表示に使うテンプレート項目を取得します。
  Future<List<TemplateItem>> listItems() => _listItems(this);
}
