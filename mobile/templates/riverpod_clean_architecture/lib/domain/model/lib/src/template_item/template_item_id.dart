/// テンプレート項目を一意に識別する値です。
final class TemplateItemId {
  TemplateItemId(String value) : value = value.trim() {
    if (this.value.isEmpty) {
      throw ArgumentError.value(value, 'value', 'IDは空にできません。');
    }
  }

  final String value;

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is TemplateItemId && value == other.value;

  @override
  int get hashCode => value.hashCode;

  @override
  String toString() => value;
}
