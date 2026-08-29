use thiserror::Error;

/// Domainから見たRepository操作の失敗理由を表す。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RepositoryError {
    #[error("entity was not found")]
    NotFound,
    #[error("an entity with the same unique value already exists")]
    AlreadyExists,
    #[error("a listing request for the same source URL already exists")]
    DuplicateListingRequest,
    #[error("stored data violates the domain model: {0}")]
    InvalidStoredData(String),
    #[error("repository is unavailable: {0}")]
    Unavailable(String),
}
