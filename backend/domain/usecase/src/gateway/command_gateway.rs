use async_trait::async_trait;
use domain_model::{Experience, ListingRequest, Practice, PracticeId, Source, SourceId};

use super::RepositoryError;

/// Domainの状態変更を永続化層へ委譲するCommand側Port。
#[async_trait]
pub trait CommandGateway: Send + Sync {
    /// 新しいPracticeを保存する。
    async fn insert_practice(&self, practice: &Practice) -> Result<(), RepositoryError>;

    /// 新しいSourceを保存する。
    async fn insert_source(&self, source: &Source) -> Result<(), RepositoryError>;

    /// PracticeとSourceの関連を重複なく保存する。
    async fn link_practice_source(
        &self,
        practice_id: PracticeId,
        source_id: SourceId,
    ) -> Result<(), RepositoryError>;

    /// 同じUser・PracticeのExperienceを原子的に保存し、実際の行と新規作成かを返す。
    async fn save_experience(
        &self,
        experience: &Experience,
    ) -> Result<(Experience, bool), RepositoryError>;

    /// 既存Experienceの更新可能な値を保存する。
    async fn update_experience(&self, experience: &Experience) -> Result<(), RepositoryError>;

    /// 状態に関係なく同一SourceUrlが未登録の場合だけ掲載依頼を保存する。
    async fn insert_listing_request(&self, request: &ListingRequest)
    -> Result<(), RepositoryError>;

    /// 既存掲載依頼の状態を保存する。
    async fn update_listing_request(&self, request: &ListingRequest)
    -> Result<(), RepositoryError>;
}
