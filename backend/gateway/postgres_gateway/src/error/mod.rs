use diesel::result::{DatabaseErrorKind, Error as DieselError};
use domain_usecase::gateway::RepositoryError;

const LISTING_REQUEST_SOURCE_URL_UNIQUE: &str = "listing_requests_source_url_unique";

/// Diesel固有エラーをDomain側のRepositoryErrorへ変換する。
pub(crate) struct PostgresqlGatewayErrorMapper;

impl PostgresqlGatewayErrorMapper {
    /// 一般的なDieselエラーをInfrastructure非依存のエラーへ変換する。
    pub(crate) fn map(error: DieselError) -> RepositoryError {
        match error {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                RepositoryError::AlreadyExists
            }
            DieselError::NotFound => RepositoryError::NotFound,
            other => RepositoryError::Unavailable(other.to_string()),
        }
    }

    /// SourceUrl制約違反だけを掲載依頼の業務エラーへ変換する。
    pub(crate) fn map_listing_request_insert(error: DieselError) -> RepositoryError {
        match error {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, information)
                if information.constraint_name() == Some(LISTING_REQUEST_SOURCE_URL_UNIQUE) =>
            {
                RepositoryError::DuplicateListingRequest
            }
            other => Self::map(other),
        }
    }
}
