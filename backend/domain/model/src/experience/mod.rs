mod experience_id;
mod experience_note;

use chrono::{DateTime, Utc};

use crate::{PracticeId, UserId};

pub use experience_id::ExperienceId;
pub use experience_note::{ExperienceNote, ExperienceNoteError};

/// ユーザーがPracticeを実際に試した記録を表すAggregate。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Experience {
    id: ExperienceId,
    practice_id: PracticeId,
    user_id: UserId,
    note: Option<ExperienceNote>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Experience {
    /// Practiceと投稿者を固定して新しいExperienceを作成する。
    pub fn new(practice_id: PracticeId, user_id: UserId, note: Option<ExperienceNote>) -> Self {
        let now = Utc::now();
        Self {
            id: ExperienceId::generate(),
            practice_id,
            user_id,
            note,
            created_at: now,
            updated_at: now,
        }
    }

    /// 永続化済みの値からExperienceを復元する。
    pub fn restore(
        id: ExperienceId,
        practice_id: PracticeId,
        user_id: UserId,
        note: Option<ExperienceNote>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            practice_id,
            user_id,
            note,
            created_at,
            updated_at,
        }
    }

    /// Experienceを識別するIDを返す。
    pub fn id(&self) -> ExperienceId {
        self.id
    }

    /// 試したPracticeのIDを返す。
    pub fn practice_id(&self) -> PracticeId {
        self.practice_id
    }

    /// Experienceを投稿したUserのIDを返す。
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    /// 任意の短い補足コメントを返す。
    pub fn note(&self) -> Option<&ExperienceNote> {
        self.note.as_ref()
    }

    /// Experienceが作成されたUTC日時を返す。
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Experienceが最後に更新されたUTC日時を返す。
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// 投稿者とPracticeを維持したまま補足コメントを更新する。
    pub fn update_note(&mut self, note: Option<ExperienceNote>) {
        self.note = note;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Note更新でExperienceの投稿者とPracticeが変わらないことを確認する。
    #[test]
    fn update_note_keeps_owner_and_practice() {
        let mut experience = Experience::new(
            PracticeId::generate(),
            UserId::generate(),
            Some(ExperienceNote::try_from("役に立った").unwrap()),
        );
        let practice_id = experience.practice_id();
        let user_id = experience.user_id();

        experience.update_note(None);

        assert_eq!(experience.practice_id(), practice_id);
        assert_eq!(experience.user_id(), user_id);
        assert_eq!(experience.note(), None);
    }
}
