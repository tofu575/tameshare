use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;
use domain_model::{ListingRequest, ListingRequestId, ListingRequestStatus, SourceUrl};
use domain_usecase::gateway::{ListingRequestRepository, RepositoryError};

use crate::{
    error::PostgresRepositoryErrorMapper, postgres_repository::PostgresRepository,
    row::ListingRequestRow, schema::listing_requests,
};

/// ListingRequestStatusをDBのCHECK制約と同じ文字列へ変換する。
fn status_as_str(status: ListingRequestStatus) -> &'static str {
    match status {
        ListingRequestStatus::Pending => "pending",
        ListingRequestStatus::Accepted => "accepted",
        ListingRequestStatus::Rejected => "rejected",
    }
}

#[async_trait]
impl ListingRequestRepository for PostgresRepository {
    /// ListingRequestを保存し、SourceUrl制約違反だけを専用エラーへ変換する。
    async fn insert(&self, request: &ListingRequest) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        diesel::insert_into(listing_requests::table)
            .values((
                listing_requests::id.eq(*request.id().as_uuid()),
                listing_requests::source_url.eq(request.source_url().as_str()),
                listing_requests::user_id.eq(*request.user_id().as_uuid()),
                listing_requests::status.eq(status_as_str(request.status())),
                listing_requests::created_at.eq(request.created_at()),
                listing_requests::updated_at.eq(request.updated_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresRepositoryErrorMapper::map_listing_request_insert)?;
        Ok(())
    }

    /// DBからIDに一致するListingRequestを取得してDomain Modelへ復元する。
    async fn find(&self, id: ListingRequestId) -> Result<Option<ListingRequest>, RepositoryError> {
        let mut connection = self.connection().await?;
        let row = listing_requests::table
            .find(*id.as_uuid())
            .select((
                listing_requests::id,
                listing_requests::source_url,
                listing_requests::user_id,
                listing_requests::status,
                listing_requests::created_at,
                listing_requests::updated_at,
            ))
            .first::<ListingRequestRow>(&mut connection)
            .await
            .optional()
            .map_err(PostgresRepositoryErrorMapper::map)?;
        row.map(TryInto::try_into).transpose()
    }

    /// DBから完全一致するSourceUrlのListingRequestを取得する。
    async fn find_by_source_url(
        &self,
        source_url: &SourceUrl,
    ) -> Result<Option<ListingRequest>, RepositoryError> {
        let mut connection = self.connection().await?;
        let row = listing_requests::table
            .filter(listing_requests::source_url.eq(source_url.as_str()))
            .select((
                listing_requests::id,
                listing_requests::source_url,
                listing_requests::user_id,
                listing_requests::status,
                listing_requests::created_at,
                listing_requests::updated_at,
            ))
            .first::<ListingRequestRow>(&mut connection)
            .await
            .optional()
            .map_err(PostgresRepositoryErrorMapper::map)?;
        row.map(TryInto::try_into).transpose()
    }

    /// ListingRequestの状態と更新日時だけを保存する。
    async fn update(&self, request: &ListingRequest) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        let changed = diesel::update(listing_requests::table.find(*request.id().as_uuid()))
            .set((
                listing_requests::status.eq(status_as_str(request.status())),
                listing_requests::updated_at.eq(request.updated_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresRepositoryErrorMapper::map)?;
        if changed == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }
}
