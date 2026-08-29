use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;
use domain_model::{
    Experience, ExperienceId, ListingRequest, ListingRequestId, Practice, PracticeId, Source,
    SourceId, SourceUrl,
};
use domain_usecase::gateway::{QueryGateway, RepositoryError};

use crate::{
    error::PostgresqlGatewayErrorMapper,
    postgresql_gateway::PostgresqlGateway,
    row::{ExperienceRow, ListingRequestRow, PracticeRow, SourceRow},
    schema::{experiences, listing_requests, practice_sources, practices, sources},
};

#[async_trait]
impl QueryGateway for PostgresqlGateway {
    /// Practiceを作成日時とIDの降順で安定してページ取得する。
    async fn list_practices(
        &self,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Practice>, RepositoryError> {
        let mut connection = self.connection().await?;
        practices::table
            .select((practices::id, practices::title, practices::created_at))
            .order((practices::created_at.desc(), practices::id.desc()))
            .limit(i64::from(limit))
            .offset(offset as i64)
            .load::<PracticeRow>(&mut connection)
            .await
            .map(|rows| rows.into_iter().map(Into::into).collect())
            .map_err(PostgresqlGatewayErrorMapper::map)
    }

    /// DBからIDに一致するPracticeを取得してDomain Modelへ復元する。
    async fn find_practice(&self, id: PracticeId) -> Result<Option<Practice>, RepositoryError> {
        let mut connection = self.connection().await?;
        practices::table
            .find(*id.as_uuid())
            .select((practices::id, practices::title, practices::created_at))
            .first::<PracticeRow>(&mut connection)
            .await
            .optional()
            .map(|row| row.map(Into::into))
            .map_err(PostgresqlGatewayErrorMapper::map)
    }

    /// DBからIDに一致するSourceを取得してDomain Modelへ復元する。
    async fn find_source(&self, id: SourceId) -> Result<Option<Source>, RepositoryError> {
        let mut connection = self.connection().await?;
        let row = sources::table
            .find(*id.as_uuid())
            .select((sources::id, sources::url, sources::created_at))
            .first::<SourceRow>(&mut connection)
            .await
            .optional()
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        row.map(TryInto::try_into).transpose()
    }

    /// 中間テーブルを介してPracticeに関連するSourceを復元する。
    async fn find_sources_by_practice(
        &self,
        practice_id: PracticeId,
    ) -> Result<Vec<Source>, RepositoryError> {
        let mut connection = self.connection().await?;
        let rows = practice_sources::table
            .inner_join(sources::table)
            .filter(practice_sources::practice_id.eq(*practice_id.as_uuid()))
            .select((sources::id, sources::url, sources::created_at))
            .load::<SourceRow>(&mut connection)
            .await
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        rows.into_iter().map(TryInto::try_into).collect()
    }

    /// PracticeのExperienceを作成日時とIDの降順で安定してページ取得する。
    async fn list_experiences_by_practice(
        &self,
        practice_id: PracticeId,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Experience>, RepositoryError> {
        let mut connection = self.connection().await?;
        let rows = experiences::table
            .filter(experiences::practice_id.eq(*practice_id.as_uuid()))
            .select((
                experiences::id,
                experiences::practice_id,
                experiences::user_id,
                experiences::note,
                experiences::created_at,
                experiences::updated_at,
            ))
            .order((experiences::created_at.desc(), experiences::id.desc()))
            .limit(i64::from(limit))
            .offset(offset as i64)
            .load::<ExperienceRow>(&mut connection)
            .await
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        rows.into_iter().map(TryInto::try_into).collect()
    }

    /// DBからIDに一致するExperienceを取得してDomain Modelへ復元する。
    async fn find_experience(
        &self,
        id: ExperienceId,
    ) -> Result<Option<Experience>, RepositoryError> {
        let mut connection = self.connection().await?;
        let row = experiences::table
            .find(*id.as_uuid())
            .select((
                experiences::id,
                experiences::practice_id,
                experiences::user_id,
                experiences::note,
                experiences::created_at,
                experiences::updated_at,
            ))
            .first::<ExperienceRow>(&mut connection)
            .await
            .optional()
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        row.map(TryInto::try_into).transpose()
    }

    /// DBからIDに一致するListingRequestを取得してDomain Modelへ復元する。
    async fn find_listing_request(
        &self,
        id: ListingRequestId,
    ) -> Result<Option<ListingRequest>, RepositoryError> {
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
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        row.map(TryInto::try_into).transpose()
    }

    /// DBから完全一致するSourceUrlのListingRequestを取得する。
    async fn find_listing_request_by_source_url(
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
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        row.map(TryInto::try_into).transpose()
    }
}
