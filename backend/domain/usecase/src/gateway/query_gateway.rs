use async_trait::async_trait;
use domain_model::{
    Experience, ExperienceId, ListingRequest, ListingRequestId, Practice, PracticeId, Source,
    SourceId, SourceUrl,
};

use super::RepositoryError;

/// Domainの参照処理を永続化層へ委譲するQuery側Port。
#[async_trait]
pub trait QueryGateway: Send + Sync {
    /// Practiceを新着順でページ取得する。
    async fn list_practices(
        &self,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Practice>, RepositoryError>;

    /// IDに一致するPracticeを取得する。
    async fn find_practice(&self, id: PracticeId) -> Result<Option<Practice>, RepositoryError>;

    /// IDに一致するSourceを取得する。
    async fn find_source(&self, id: SourceId) -> Result<Option<Source>, RepositoryError>;

    /// Practiceに関連付けられたSourceを取得する。
    async fn find_sources_by_practice(
        &self,
        practice_id: PracticeId,
    ) -> Result<Vec<Source>, RepositoryError>;

    /// Practiceに紐づくExperienceを新着順でページ取得する。
    async fn list_experiences_by_practice(
        &self,
        practice_id: PracticeId,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Experience>, RepositoryError>;

    /// IDに一致するExperienceを取得する。
    async fn find_experience(
        &self,
        id: ExperienceId,
    ) -> Result<Option<Experience>, RepositoryError>;

    /// IDに一致する掲載依頼を取得する。
    async fn find_listing_request(
        &self,
        id: ListingRequestId,
    ) -> Result<Option<ListingRequest>, RepositoryError>;

    /// 完全一致するSourceUrlの掲載依頼を取得する。
    async fn find_listing_request_by_source_url(
        &self,
        source_url: &SourceUrl,
    ) -> Result<Option<ListingRequest>, RepositoryError>;
}
