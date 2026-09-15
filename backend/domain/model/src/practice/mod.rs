mod practice_id;

use chrono::{DateTime, Utc};

pub use practice_id::PracticeId;

/// サービス上で紹介する、短時間で実践可能な方法を表すAggregate。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Practice {
    id: PracticeId,
    title: String,
    created_at: DateTime<Utc>,
}

impl Practice {
    /// 新しいPracticeをUUID v7と現在UTC時刻で作成する。
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: PracticeId::generate(),
            title: title.into(),
            created_at: Utc::now(),
        }
    }

    /// 永続化済みの値からPracticeを復元する。
    pub fn restore(id: PracticeId, title: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            created_at,
        }
    }

    /// Practiceを識別するIDを返す。
    pub fn id(&self) -> PracticeId {
        self.id
    }

    /// 実践方法のタイトルを返す。
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Practiceが登録されたUTC日時を返す。
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
