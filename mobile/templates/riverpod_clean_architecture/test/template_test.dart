import 'package:flutter_test/flutter_test.dart';
import 'package:mobile_template/wire/interactor_factory.dart';

void main() {
  test('DummyGateway is connected to ItemInteractor', () async {
    final items = await buildItemInteractor().listItems();

    expect(items, hasLength(1));
    expect(items.single.title.value, 'Dummy item');
  });
}
