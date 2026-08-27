import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:usecase/usecase.dart';

import '../pages/item_list/item_list_page.dart';
import '../providers/item_provider.dart';

/// [itemInteractor]をPresentationへ注入し、テンプレート一覧を表示します。
class TemplateApp extends StatelessWidget {
  const TemplateApp({required this.itemInteractor, super.key});

  final ItemInteractor itemInteractor;

  @override
  Widget build(BuildContext context) {
    return ProviderScope(
      overrides: [itemInteractorProvider.overrideWithValue(itemInteractor)],
      child: MaterialApp(
        title: 'Template App',
        theme: ThemeData(useMaterial3: true),
        home: const ItemListPage(),
      ),
    );
  }
}
