use chrono::{DateTime, Utc};
use domain_model::{Source, SourceId, SourceUrl};
use domain_usecase::gateway::RepositoryError;
use uuid::Uuid;

/// sourcesテーブルから読み取るDiesel DTO。
#[derive(diesel::Queryable)]
pub(crate) struct SourceRow {
    id: Uuid,
    url: String,
    created_at: DateTime<Utc>,
}

impl TryFrom<SourceRow> for Source {
    type Error = RepositoryError;

    /// DB DTOのURLを再検証してSource Aggregateへ復元する。
    fn try_from(row: SourceRow) -> Result<Self, Self::Error> {
        let url = SourceUrl::try_from(row.url)
            .map_err(|error| RepositoryError::InvalidStoredData(error.to_string()))?;
        Ok(Self::restore(
            SourceId::from_uuid(row.id),
            url,
            row.created_at,
        ))
    }
}
