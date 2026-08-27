/// テンプレート項目に表示するタイトルです。
final class TemplateItemTitle {
  TemplateItemTitle(String value) : value = value.trim() {
    if (this.value.isEmpty) {
      throw ArgumentError.value(value, 'value', 'タイトルは空にできません。');
    }
  }

  final String value;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TemplateItemTitle && value == other.value;

  @override
  int get hashCode => value.hashCode;

  @override
  String toString() => value;
}
