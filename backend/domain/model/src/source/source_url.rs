use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;
use url::Url;

/// 正規化せず原文を保持する、絶対HTTP/HTTPS形式のSource URL。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceUrl(String);

impl SourceUrl {
    /// 永続化や外部出力に利用する元のURL文字列を返す。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// SourceUrlの生成に失敗した理由を表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SourceUrlError {
    #[error("source URL must not be empty")]
    Empty,
    #[error("source URL must be an absolute HTTP or HTTPS URL with a host")]
    Invalid,
}

impl TryFrom<String> for SourceUrl {
    type Error = SourceUrlError;

    /// URLを正規化せず、絶対HTTP/HTTPS URLであることだけを検証する。
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(SourceUrlError::Empty);
        }
        let parsed = Url::parse(&value).map_err(|_| SourceUrlError::Invalid)?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
            return Err(SourceUrlError::Invalid);
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for SourceUrl {
    type Error = SourceUrlError;

    /// 借用文字列からSourceUrlを検証して作成する。
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl fmt::Display for SourceUrl {
    /// SourceUrlが保持する元の文字列を表示する。
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Serialize for SourceUrl {
    /// SourceUrlを元のURL文字列として直列化する。
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SourceUrl {
    /// 復号した文字列を再検証してSourceUrlを復元する。
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

    /// SourceUrlの許可形式・拒否形式・非正規化をまとめて確認する。
    #[test]
    fn validates_without_normalizing() {
        let with_slash = SourceUrl::try_from("https://example.com/article/").unwrap();
        let without_slash = SourceUrl::try_from("https://example.com/article").unwrap();

        assert_ne!(with_slash, without_slash);
        assert!(SourceUrl::try_from("http://example.com").is_ok());
        assert_eq!(SourceUrl::try_from(""), Err(SourceUrlError::Empty));
        assert_eq!(
            SourceUrl::try_from("example.com"),
            Err(SourceUrlError::Invalid)
        );
        assert_eq!(
            SourceUrl::try_from("ftp://example.com"),
            Err(SourceUrlError::Invalid)
        );
    }
}
