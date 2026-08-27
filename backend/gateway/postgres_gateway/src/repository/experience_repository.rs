use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;
use domain_model::{Experience, ExperienceId, ExperienceNote, PracticeId};
use domain_usecase::gateway::{ExperienceRepository, RepositoryError};

use crate::{
    error::PostgresRepositoryErrorMapper, postgres_repository::PostgresRepository,
    row::ExperienceRow, schema::experiences,
};

#[async_trait]
impl ExperienceRepository for PostgresRepository {
    /// PracticeのExperienceを作成日時とIDの降順で安定してページ取得する。
    async fn list_by_practice(
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
            .map_err(PostgresRepositoryErrorMapper::map)?;
        rows.into_iter().map(TryInto::try_into).collect()
    }

    /// ExperienceをDomain ModelからDB列へ変換して保存する。
    async fn insert(&self, experience: &Experience) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        diesel::insert_into(experiences::table)
            .values((
                experiences::id.eq(*experience.id().as_uuid()),
                experiences::practice_id.eq(*experience.practice_id().as_uuid()),
                experiences::user_id.eq(*experience.user_id().as_uuid()),
                experiences::note.eq(experience.note().map(ExperienceNote::as_str)),
                experiences::created_at.eq(experience.created_at()),
                experiences::updated_at.eq(experience.updated_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresRepositoryErrorMapper::map)?;
        Ok(())
    }

    /// DBからIDに一致するExperienceを取得してDomain Modelへ復元する。
    async fn find(&self, id: ExperienceId) -> Result<Option<Experience>, RepositoryError> {
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
            .map_err(PostgresRepositoryErrorMapper::map)?;
        row.map(TryInto::try_into).transpose()
    }

    /// Experienceの更新可能なNoteと更新日時だけを保存する。
    async fn update(&self, experience: &Experience) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        let changed = diesel::update(experiences::table.find(*experience.id().as_uuid()))
            .set((
                experiences::note.eq(experience.note().map(ExperienceNote::as_str)),
                experiences::updated_at.eq(experience.updated_at()),
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
