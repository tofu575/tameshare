use serde::{Deserialize, Serialize};

/// 掲載依頼のMVP上の処理状態を表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListingRequestStatus {
    Pending,
    Accepted,
    Rejected,
}
