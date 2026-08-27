use async_trait::async_trait;
use domain_model::{PracticeId, Source, SourceId};

use super::RepositoryError;

/// Aggregateのライフサイクルを結合せずPracticeとSourceのN:M関係を管理する。
#[async_trait]
pub trait PracticeSourceRepository: Send + Sync {
    /// PracticeとSourceの関連を重複なく保存する。
    async fn link(
        &self,
        practice_id: PracticeId,
        source_id: SourceId,
    ) -> Result<(), RepositoryError>;

    /// Practiceに関連付けられたSourceを取得する。
    async fn find_sources(&self, practice_id: PracticeId) -> Result<Vec<Source>, RepositoryError>;
}
