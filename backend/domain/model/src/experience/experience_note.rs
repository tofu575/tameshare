use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

/// Experienceへ任意で付ける、URLを含まない100文字以内の補足コメント。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperienceNote(String);

impl ExperienceNote {
    pub const MAX_CHARS: usize = 100;

    /// 補足コメントの文字列を返す。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// MVPでURLとみなす明示的な文字列を含むか判定する。
    fn contains_url(value: &str) -> bool {
        let lowercase = value.to_ascii_lowercase();
        ["http://", "https://", "www."]
            .iter()
            .any(|marker| lowercase.contains(marker))
    }
}

/// ExperienceNoteの生成に失敗した理由を表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ExperienceNoteError {
    #[error("experience note must contain at most 100 characters")]
    TooLong,
    #[error("experience note must not contain a URL")]
    ContainsUrl,
}

impl TryFrom<String> for ExperienceNote {
    type Error = ExperienceNoteError;

    /// Unicode文字数とMVPのURLマーカーを検証してNoteを作成する。
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > Self::MAX_CHARS {
            return Err(ExperienceNoteError::TooLong);
        }
        if Self::contains_url(&value) {
            return Err(ExperienceNoteError::ContainsUrl);
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for ExperienceNote {
    type Error = ExperienceNoteError;

    /// 借用文字列からExperienceNoteを検証して作成する。
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl fmt::Display for ExperienceNote {
    /// ExperienceNoteが保持する文字列を表示する。
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Serialize for ExperienceNote {
    /// ExperienceNoteを文字列として直列化する。
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ExperienceNote {
    /// 復号した文字列を再検証してExperienceNoteを復元する。
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::try_from(value).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 100 Unicode文字の境界とURLマーカー拒否をまとめて確認する。
    #[test]
    fn validates_character_count_and_url_markers() {
        assert!(ExperienceNote::try_from("あ".repeat(100)).is_ok());
        assert_eq!(
            ExperienceNote::try_from("あ".repeat(101)),
            Err(ExperienceNoteError::TooLong)
        );
        for value in [
            "see http://example.com",
            "see HTTPS://example.com",
            "see www.example.com",
        ] {
            assert_eq!(
                ExperienceNote::try_from(value),
                Err(ExperienceNoteError::ContainsUrl)
            );
        }
    }
}
