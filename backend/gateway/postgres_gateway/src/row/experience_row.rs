use chrono::{DateTime, Utc};
use domain_model::{Experience, ExperienceId, ExperienceNote, PracticeId, UserId};
use domain_usecase::gateway::RepositoryError;
use uuid::Uuid;

/// experiencesテーブルから読み取るDiesel DTO。
#[derive(diesel::Queryable)]
pub(crate) struct ExperienceRow {
    id: Uuid,
    practice_id: Uuid,
    user_id: Uuid,
    note: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<ExperienceRow> for Experience {
    type Error = RepositoryError;

    /// DB DTOのNoteを再検証してExperience Aggregateへ復元する。
    fn try_from(row: ExperienceRow) -> Result<Self, Self::Error> {
        let note = row
            .note
            .map(ExperienceNote::try_from)
            .transpose()
            .map_err(|error| RepositoryError::InvalidStoredData(error.to_string()))?;
        Ok(Self::restore(
            ExperienceId::from_uuid(row.id),
            PracticeId::from_uuid(row.practice_id),
            UserId::from_uuid(row.user_id),
            note,
            row.created_at,
            row.updated_at,
        ))
    }
}
