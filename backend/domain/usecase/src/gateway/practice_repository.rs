use async_trait::async_trait;
use domain_model::{Practice, PracticeId};

use super::RepositoryError;

/// Practice Aggregateの永続化境界を定義する。
#[async_trait]
pub trait PracticeRepository: Send + Sync {
    /// Practiceを新着順でページ取得する。
    async fn list(&self, limit: u32, offset: u64) -> Result<Vec<Practice>, RepositoryError>;

    /// 新しいPracticeを保存する。
    async fn insert(&self, practice: &Practice) -> Result<(), RepositoryError>;

    /// IDに一致するPracticeを取得する。
    async fn find(&self, id: PracticeId) -> Result<Option<Practice>, RepositoryError>;
}
