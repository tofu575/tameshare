use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use domain_model::{
    Experience, ExperienceNote, ListingRequest, ListingRequestStatus, Practice, PracticeId, Source,
    SourceId,
};
use domain_usecase::gateway::{CommandGateway, RepositoryError};

use crate::{
    error::PostgresqlGatewayErrorMapper,
    postgresql_gateway::PostgresqlGateway,
    row::ExperienceRow,
    schema::{experiences, listing_requests, practice_sources, practices, sources},
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
impl CommandGateway for PostgresqlGateway {
    /// PracticeをDomain ModelからDB列へ変換して保存する。
    async fn insert_practice(&self, practice: &Practice) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        diesel::insert_into(practices::table)
            .values((
                practices::id.eq(*practice.id().as_uuid()),
                practices::title.eq(practice.title()),
                practices::created_at.eq(practice.created_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        Ok(())
    }

    /// SourceをDomain ModelからDB列へ変換して保存する。
    async fn insert_source(&self, source: &Source) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        diesel::insert_into(sources::table)
            .values((
                sources::id.eq(*source.id().as_uuid()),
                sources::url.eq(source.url().as_str()),
                sources::created_at.eq(source.created_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        Ok(())
    }

    /// 中間テーブルの複合主キーを利用して関連を冪等に保存する。
    async fn link_practice_source(
        &self,
        practice_id: PracticeId,
        source_id: SourceId,
    ) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        diesel::insert_into(practice_sources::table)
            .values((
                practice_sources::practice_id.eq(*practice_id.as_uuid()),
                practice_sources::source_id.eq(*source_id.as_uuid()),
            ))
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .await
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        Ok(())
    }

    /// User・Practiceの一意制約を使いExperienceを原子的に保存する。
    async fn save_experience(
        &self,
        experience: &Experience,
    ) -> Result<(Experience, bool), RepositoryError> {
        let mut connection = self.connection().await?;
        let row = diesel::insert_into(experiences::table)
            .values((
                experiences::id.eq(*experience.id().as_uuid()),
                experiences::practice_id.eq(*experience.practice_id().as_uuid()),
                experiences::user_id.eq(*experience.user_id().as_uuid()),
                experiences::note.eq(experience.note().map(ExperienceNote::as_str)),
                experiences::created_at.eq(experience.created_at()),
                experiences::updated_at.eq(experience.updated_at()),
            ))
            .on_conflict((experiences::user_id, experiences::practice_id))
            .do_update()
            .set((
                experiences::note.eq(experience.note().map(ExperienceNote::as_str)),
                experiences::updated_at.eq(experience.updated_at()),
            ))
            .returning((
                experiences::id,
                experiences::practice_id,
                experiences::user_id,
                experiences::note,
                experiences::created_at,
                experiences::updated_at,
            ))
            .get_result::<ExperienceRow>(&mut connection)
            .await
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        let saved: Experience = row.try_into()?;
        let created = saved.id() == experience.id();
        Ok((saved, created))
    }

    /// Experienceの更新可能なNoteと更新日時だけを保存する。
    async fn update_experience(&self, experience: &Experience) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        let changed = diesel::update(experiences::table.find(*experience.id().as_uuid()))
            .set((
                experiences::note.eq(experience.note().map(ExperienceNote::as_str)),
                experiences::updated_at.eq(experience.updated_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        if changed == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }

    /// ListingRequestを保存し、SourceUrl制約違反だけを専用エラーへ変換する。
    async fn insert_listing_request(
        &self,
        request: &ListingRequest,
    ) -> Result<(), RepositoryError> {
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
            .map_err(PostgresqlGatewayErrorMapper::map_listing_request_insert)?;
        Ok(())
    }

    /// ListingRequestの状態と更新日時だけを保存する。
    async fn update_listing_request(
        &self,
        request: &ListingRequest,
    ) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        let changed = diesel::update(listing_requests::table.find(*request.id().as_uuid()))
            .set((
                listing_requests::status.eq(status_as_str(request.status())),
                listing_requests::updated_at.eq(request.updated_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresqlGatewayErrorMapper::map)?;
        if changed == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }
}
