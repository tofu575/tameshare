import 'template_item_id.dart';
import 'template_item_title.dart';

/// 一覧に表示するテンプレート項目です。
final class TemplateItem {
  const TemplateItem({required this.id, required this.title});

  final TemplateItemId id;
  final TemplateItemTitle title;
}
