use serde::{Deserialize, Serialize};

use crate::{
    user::PartialUser,
};

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GiftPrivateResponse {
    pub code: String,
    pub duration_months: u64,
    // ISO8601
    pub created_at: jiff::Timestamp,
    pub created_by: PartialUser,
    // ISO8601
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemed_at: Option<jiff::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemed_by: Option<PartialUser>,
}
