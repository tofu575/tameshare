use async_trait::async_trait;
use domain_model::{ListingRequest, ListingRequestId, SourceUrl};

use super::RepositoryError;

/// ListingRequest AggregateとSourceUrl重複禁止の永続化境界を定義する。
#[async_trait]
pub trait ListingRequestRepository: Send + Sync {
    /// 状態に関係なく同一SourceUrlが未登録の場合だけ掲載依頼を保存する。
    async fn insert(&self, request: &ListingRequest) -> Result<(), RepositoryError>;

    /// IDに一致する掲載依頼を取得する。
    async fn find(&self, id: ListingRequestId) -> Result<Option<ListingRequest>, RepositoryError>;

    /// 完全一致するSourceUrlの掲載依頼を取得する。
    async fn find_by_source_url(
        &self,
        source_url: &SourceUrl,
    ) -> Result<Option<ListingRequest>, RepositoryError>;

    /// 既存掲載依頼の状態を保存する。
    async fn update(&self, request: &ListingRequest) -> Result<(), RepositoryError>;
}
