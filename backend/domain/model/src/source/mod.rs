mod source_id;
mod source_url;

use chrono::{DateTime, Utc};

pub use source_id::SourceId;
pub use source_url::{SourceUrl, SourceUrlError};

/// Practiceの根拠・出典となるWeb情報源を表すAggregate。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    id: SourceId,
    url: SourceUrl,
    created_at: DateTime<Utc>,
}

impl Source {
    /// 検証済みURLから新しいSourceを作成する。
    pub fn new(url: SourceUrl) -> Self {
        Self {
            id: SourceId::generate(),
            url,
            created_at: Utc::now(),
        }
    }

    /// 永続化済みの値からSourceを復元する。
    pub fn restore(id: SourceId, url: SourceUrl, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            url,
            created_at,
        }
    }

    /// Sourceを識別するIDを返す。
    pub fn id(&self) -> SourceId {
        self.id
    }

    /// 出典を識別する検証済みURLを返す。
    pub fn url(&self) -> &SourceUrl {
        &self.url
    }

    /// Sourceが登録されたUTC日時を返す。
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
