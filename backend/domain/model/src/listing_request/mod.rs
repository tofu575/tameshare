mod listing_request_id;
mod listing_request_status;

use chrono::{DateTime, Utc};

use crate::{SourceUrl, UserId};

pub use listing_request_id::ListingRequestId;
pub use listing_request_status::ListingRequestStatus;

/// ユーザーがSource URLの掲載を運営へ依頼するAggregate。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListingRequest {
    id: ListingRequestId,
    source_url: SourceUrl,
    user_id: UserId,
    status: ListingRequestStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ListingRequest {
    /// 検証済みURLに対するPending状態の掲載依頼を作成する。
    pub fn new(source_url: SourceUrl, user_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            id: ListingRequestId::generate(),
            source_url,
            user_id,
            status: ListingRequestStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    /// 永続化済みの値からListingRequestを復元する。
    pub fn restore(
        id: ListingRequestId,
        source_url: SourceUrl,
        user_id: UserId,
        status: ListingRequestStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            source_url,
            user_id,
            status,
            created_at,
            updated_at,
        }
    }

    /// ListingRequestを識別するIDを返す。
    pub fn id(&self) -> ListingRequestId {
        self.id
    }

    /// 掲載を依頼されたSource URLを返す。
    pub fn source_url(&self) -> &SourceUrl {
        &self.source_url
    }

    /// 掲載依頼を作成したUserのIDを返す。
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    /// 掲載依頼の現在状態を返す。
    pub fn status(&self) -> ListingRequestStatus {
        self.status
    }

    /// 掲載依頼が作成されたUTC日時を返す。
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// 掲載依頼が最後に更新されたUTC日時を返す。
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// 掲載依頼をAccepted状態へ変更する。
    pub fn accept(&mut self) {
        self.status = ListingRequestStatus::Accepted;
        self.updated_at = Utc::now();
    }

    /// 掲載依頼をRejected状態へ変更する。
    pub fn reject(&mut self) {
        self.status = ListingRequestStatus::Rejected;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 新規掲載依頼が必ずPending状態で始まることを確認する。
    #[test]
    fn starts_pending() {
        let request = ListingRequest::new(
            SourceUrl::try_from("https://example.com/article").unwrap(),
            UserId::generate(),
        );

        assert_eq!(request.status(), ListingRequestStatus::Pending);
    }
}
