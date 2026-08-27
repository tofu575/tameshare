use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;
use domain_model::{Practice, PracticeId};
use domain_usecase::gateway::{PracticeRepository, RepositoryError};

use crate::{
    error::PostgresRepositoryErrorMapper, postgres_repository::PostgresRepository,
    row::PracticeRow, schema::practices,
};

#[async_trait]
impl PracticeRepository for PostgresRepository {
    /// PracticeをDomain ModelからDB列へ変換して保存する。
    async fn insert(&self, practice: &Practice) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        diesel::insert_into(practices::table)
            .values((
                practices::id.eq(*practice.id().as_uuid()),
                practices::title.eq(practice.title()),
                practices::created_at.eq(practice.created_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresRepositoryErrorMapper::map)?;
        Ok(())
    }

    /// DBからIDに一致するPracticeを取得してDomain Modelへ復元する。
    async fn find(&self, id: PracticeId) -> Result<Option<Practice>, RepositoryError> {
        let mut connection = self.connection().await?;
        practices::table
            .find(*id.as_uuid())
            .select((practices::id, practices::title, practices::created_at))
            .first::<PracticeRow>(&mut connection)
            .await
            .optional()
            .map(|row| row.map(Into::into))
            .map_err(PostgresRepositoryErrorMapper::map)
    }
}
