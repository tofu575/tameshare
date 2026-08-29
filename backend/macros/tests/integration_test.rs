use macros::TypedString;
use macros::TypedUuid;
use uuid::{Uuid, Version};

#[derive(Debug, Clone, PartialEq, Eq, TypedUuid)]
/// TypedUuid deriveを検証するテスト専用ID。
struct TestId(Uuid);

#[test]
/// UUID v7生成と文字列往復を確認する。
fn typed_uuid_generates_and_round_trips_an_id() {
    let id = TestId::generate();

    assert_eq!(id.as_uuid().get_version(), Some(Version::SortRand));
    assert_eq!(TestId::try_from(id.to_string()).unwrap(), id);
    assert_eq!(TestId::from_uuid(*id.as_uuid()), id);
}

#[derive(Debug, Clone, PartialEq, Eq, TypedString)]
#[typed_string(min = 2, max = 4, trim)]
/// TypedString deriveを検証するテスト専用Value Object。
struct TestName(String);

#[test]
/// 文字数検証とtrimが生成されることを確認する。
fn typed_string_validates_character_count_and_trims() {
    let name = TestName::try_from("  地図  ".to_owned()).unwrap();

    assert_eq!(name.as_str(), "地図");
    assert_eq!(name.to_string(), "地図");
    assert_eq!(TestName::MIN_CHARS, 2);
    assert_eq!(TestName::MAX_CHARS, 4);
    assert!(TestName::try_from("a".to_owned()).is_err());
    assert!(TestName::try_from("abcde".to_owned()).is_err());
}
