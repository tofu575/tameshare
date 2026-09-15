use chrono::{DateTime, Utc};
use domain_model::{ListingRequest, ListingRequestId, ListingRequestStatus, SourceUrl, UserId};
use domain_usecase::gateway::RepositoryError;
use uuid::Uuid;

/// listing_requestsテーブルから読み取るDiesel DTO。
#[derive(diesel::Queryable)]
pub(crate) struct ListingRequestRow {
    id: Uuid,
    source_url: String,
    user_id: Uuid,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<ListingRequestRow> for ListingRequest {
    type Error = RepositoryError;

    /// DB DTOのURLと状態を再検証してListingRequest Aggregateへ復元する。
    fn try_from(row: ListingRequestRow) -> Result<Self, Self::Error> {
        let status = match row.status.as_str() {
            "pending" => ListingRequestStatus::Pending,
            "accepted" => ListingRequestStatus::Accepted,
            "rejected" => ListingRequestStatus::Rejected,
            value => {
                return Err(RepositoryError::InvalidStoredData(format!(
                    "unknown listing request status: {value}"
                )));
            }
        };
        let source_url = SourceUrl::try_from(row.source_url)
            .map_err(|error| RepositoryError::InvalidStoredData(error.to_string()))?;
        Ok(Self::restore(
            ListingRequestId::from_uuid(row.id),
            source_url,
            UserId::from_uuid(row.user_id),
            status,
            row.created_at,
            row.updated_at,
        ))
    }
}
