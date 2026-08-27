import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:model/model.dart';
import 'package:presentation/presentation.dart';
import 'package:usecase/usecase.dart';

final class _ItemGatewayStub implements ItemGateway {
  @override
  Future<List<TemplateItem>> findAll() async => <TemplateItem>[
    TemplateItem(
      id: TemplateItemId('item-id'),
      title: TemplateItemTitle('Stub item'),
    ),
  ];
}

void main() {
  testWidgets('shows items loaded through the provider', (tester) async {
    await tester.pumpWidget(
      TemplateApp(
        itemInteractor: ItemInteractor(itemGateway: _ItemGatewayStub()),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Stub item'), findsOneWidget);
    expect(find.byType(ListTile), findsOneWidget);
  });
}
