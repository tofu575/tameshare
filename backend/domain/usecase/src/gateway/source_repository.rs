use async_trait::async_trait;
use domain_model::{Source, SourceId};

use super::RepositoryError;

/// Source Aggregateの永続化境界を定義する。
#[async_trait]
pub trait SourceRepository: Send + Sync {
    /// 新しいSourceを保存する。
    async fn insert(&self, source: &Source) -> Result<(), RepositoryError>;

    /// IDに一致するSourceを取得する。
    async fn find(&self, id: SourceId) -> Result<Option<Source>, RepositoryError>;
}
