import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:model/model.dart';

import '../../providers/item_provider.dart';

/// テンプレート項目の読込状態と一覧を表示し、再読込操作を受け持ちます。
class ItemListPage extends ConsumerWidget {
  const ItemListPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final items = ref.watch(itemsProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Template items'),
        actions: <Widget>[
          IconButton(
            tooltip: 'Reload',
            onPressed: () => ref.read(itemsProvider.notifier).reload(),
            icon: const Icon(Icons.refresh),
          ),
        ],
      ),
      body: items.when(
        data: _ItemList.new,
        error: (error, stackTrace) => _LoadError(
          onRetry: () => ref.read(itemsProvider.notifier).reload(),
        ),
        loading: () => const Center(child: CircularProgressIndicator()),
      ),
    );
  }
}

/// [items]を一覧表示し、空の場合は空状態を表示します。
class _ItemList extends StatelessWidget {
  const _ItemList(this.items);

  final List<TemplateItem> items;

  @override
  Widget build(BuildContext context) {
    if (items.isEmpty) {
      return const Center(child: Text('No items'));
    }

    return ListView.builder(
      itemCount: items.length,
      itemBuilder: (context, index) {
        final item = items[index];
        return ListTile(
          key: ValueKey<String>(item.id.value),
          title: Text(item.title.value),
        );
      },
    );
  }
}

/// 読込エラーを表示し、[onRetry]による再試行を受け付けます。
class _LoadError extends StatelessWidget {
  const _LoadError({required this.onRetry});

  final VoidCallback onRetry;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: <Widget>[
          const Text('Failed to load items'),
          const SizedBox(height: 8),
          FilledButton(onPressed: onRetry, child: const Text('Retry')),
        ],
      ),
    );
  }
}
