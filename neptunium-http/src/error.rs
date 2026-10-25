use neptunium_model::time::duration::{Duration, MillisF64};
use serde::{Deserialize, Serialize};

use crate::error::{error_code::ApiErrorCode, validation_error_code::ApiValidationErrorCode};

pub mod error_code;
pub mod validation_error_code;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ApiErrorEntry {
    pub path: String,
    pub message: String,
    pub code: ApiValidationErrorCode,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ApiErrorResponse {
    pub code: ApiErrorCode,
    // I will assume that this field is always present
    // -> need to check further though, to be sure
    // https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/errors/src/HttpErrors.tsx#L24
    pub message: String,
    #[serde(default = "Vec::new")]
    pub errors: Vec<ApiErrorEntry>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ApiRateLimitedResponse {
    /// Is always `RateLimited`.
    pub code: ApiErrorCode,
    pub message: String,
    pub retry_after: Duration<MillisF64>,
    pub global: bool,
}
