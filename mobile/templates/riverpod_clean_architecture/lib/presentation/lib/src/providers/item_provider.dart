import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:model/model.dart';
import 'package:usecase/usecase.dart';

final itemInteractorProvider = Provider<ItemInteractor>((ref) {
  throw UnimplementedError('TemplateAppでItemInteractorを注入してください。');
});

final itemsProvider =
    AsyncNotifierProvider<ItemListNotifier, List<TemplateItem>>(
      ItemListNotifier.new,
    );

/// 項目一覧の読込状態と再読込操作を管理します。
final class ItemListNotifier extends AsyncNotifier<List<TemplateItem>> {
  @override
  Future<List<TemplateItem>> build() =>
      ref.watch(itemInteractorProvider).listItems();

  /// Gatewayから一覧を再取得し、読込状態を更新します。
  Future<void> reload() async {
    state = const AsyncLoading<List<TemplateItem>>();
    state = await AsyncValue.guard(ref.read(itemInteractorProvider).listItems);
  }
}
