use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;
use domain_model::{Source, SourceId};
use domain_usecase::gateway::{RepositoryError, SourceRepository};

use crate::{
    error::PostgresRepositoryErrorMapper, postgres_repository::PostgresRepository, row::SourceRow,
    schema::sources,
};

#[async_trait]
impl SourceRepository for PostgresRepository {
    /// SourceをDomain ModelからDB列へ変換して保存する。
    async fn insert(&self, source: &Source) -> Result<(), RepositoryError> {
        let mut connection = self.connection().await?;
        diesel::insert_into(sources::table)
            .values((
                sources::id.eq(*source.id().as_uuid()),
                sources::url.eq(source.url().as_str()),
                sources::created_at.eq(source.created_at()),
            ))
            .execute(&mut connection)
            .await
            .map_err(PostgresRepositoryErrorMapper::map)?;
        Ok(())
    }

    /// DBからIDに一致するSourceを取得してDomain Modelへ復元する。
    async fn find(&self, id: SourceId) -> Result<Option<Source>, RepositoryError> {
        let mut connection = self.connection().await?;
        let row = sources::table
            .find(*id.as_uuid())
            .select((sources::id, sources::url, sources::created_at))
            .first::<SourceRow>(&mut connection)
            .await
            .optional()
            .map_err(PostgresRepositoryErrorMapper::map)?;
        row.map(TryInto::try_into).transpose()
    }
}
