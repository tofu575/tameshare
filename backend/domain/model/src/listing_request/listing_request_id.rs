use macros::TypedUuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ListingRequestを他のAggregate IDと型安全に区別するUUID v7識別子。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TypedUuid)]
#[serde(transparent)]
pub struct ListingRequestId(Uuid);

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Version;

    /// ListingRequestIdがUUID v7として生成されることを確認する。
    #[test]
    fn generates_uuid_v7() {
        assert_eq!(
            ListingRequestId::generate().as_uuid().get_version(),
            Some(Version::SortRand)
        );
    }
}
