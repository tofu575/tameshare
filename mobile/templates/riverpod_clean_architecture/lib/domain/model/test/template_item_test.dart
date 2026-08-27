import 'package:model/model.dart';
import 'package:test/test.dart';

void main() {
  test('item values trim surrounding whitespace', () {
    final item = TemplateItem(
      id: TemplateItemId(' item-id '),
      title: TemplateItemTitle(' Dummy item '),
    );

    expect(item.id.value, 'item-id');
    expect(item.title.value, 'Dummy item');
  });

  test('item values reject empty text', () {
    expect(() => TemplateItemId(' '), throwsArgumentError);
    expect(() => TemplateItemTitle(' '), throwsArgumentError);
  });
}
