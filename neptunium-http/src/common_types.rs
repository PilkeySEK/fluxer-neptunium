use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct SuccessResponse {
    /// Is always `true`.
    pub success: serde_bool::True,
}
