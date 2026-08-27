use async_trait::async_trait;
use domain_model::{Experience, ExperienceId};

use super::RepositoryError;

/// Experience Aggregateの永続化境界を定義する。
#[async_trait]
pub trait ExperienceRepository: Send + Sync {
    /// 新しいExperienceを保存する。
    async fn insert(&self, experience: &Experience) -> Result<(), RepositoryError>;

    /// IDに一致するExperienceを取得する。
    async fn find(&self, id: ExperienceId) -> Result<Option<Experience>, RepositoryError>;

    /// 既存Experienceの更新可能な値を保存する。
    async fn update(&self, experience: &Experience) -> Result<(), RepositoryError>;
}
