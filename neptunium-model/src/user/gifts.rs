use serde::{Deserialize, Serialize};

use crate::{
    time::timestamp::{Timestamp, representations::Iso8601},
    user::PartialUser,
};

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GiftPrivateResponse {
    pub code: String,
    pub duration_months: u64,
    pub created_at: Timestamp<Iso8601>,
    pub created_by: PartialUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemed_at: Option<Timestamp<Iso8601>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemed_by: Option<PartialUser>,
}
