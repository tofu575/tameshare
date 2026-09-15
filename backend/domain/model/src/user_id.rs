use macros::TypedUuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 外部認証方式をDomainへ漏らさずユーザーを識別するUUID v7 ID。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TypedUuid)]
#[serde(transparent)]
pub struct UserId(Uuid);

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Version;

    /// UserIdがUUID v7として生成されることを確認する。
    #[test]
    fn generates_uuid_v7() {
        assert_eq!(
            UserId::generate().as_uuid().get_version(),
            Some(Version::SortRand)
        );
    }
}
