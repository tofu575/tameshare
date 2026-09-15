use chrono::{DateTime, Utc};
use domain_model::{Practice, PracticeId};
use uuid::Uuid;

/// practicesテーブルから読み取るDiesel DTO。
#[derive(diesel::Queryable)]
pub(crate) struct PracticeRow {
    id: Uuid,
    title: String,
    created_at: DateTime<Utc>,
}

impl From<PracticeRow> for Practice {
    /// DB DTOをPractice Aggregateへ復元する。
    fn from(row: PracticeRow) -> Self {
        Self::restore(PracticeId::from_uuid(row.id), row.title, row.created_at)
    }
}
