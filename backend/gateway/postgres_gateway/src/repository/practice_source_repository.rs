use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use domain_model::{PracticeId, Source, SourceId};
use domain_usecase::gateway::{PracticeSourceRepository, RepositoryError};

use crate::{
    error::PostgresRepositoryErrorMapper,
    postgres_repository::PostgresRepository,
    row::SourceRow,
    schema::{practice_sources, sources},
};

#[async_trait]
impl PracticeSourceRepository for PostgresRepository {
    /// 中間テーブルの複合主キーを利用して関連を冪等に保存する。
    async fn link(
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
            .map_err(PostgresRepositoryErrorMapper::map)?;
        Ok(())
    }

    /// 中間テーブルを介してPracticeに関連するSourceを復元する。
    async fn find_sources(&self, practice_id: PracticeId) -> Result<Vec<Source>, RepositoryError> {
        let mut connection = self.connection().await?;
        let rows = practice_sources::table
            .inner_join(sources::table)
            .filter(practice_sources::practice_id.eq(*practice_id.as_uuid()))
            .select((sources::id, sources::url, sources::created_at))
            .load::<SourceRow>(&mut connection)
            .await
            .map_err(PostgresRepositoryErrorMapper::map)?;
        rows.into_iter().map(TryInto::try_into).collect()
    }
}
