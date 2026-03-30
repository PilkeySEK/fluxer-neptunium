use bon::Builder;
use neptunium_model::user::auth::{MfaBackupCode, SudoVerification};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct GetMfaBackupCodes {
    #[serde(flatten)]
    pub auth: SudoVerification,
    /// Whether to regenerate backup codes.
    #[builder(default = false)]
    pub regenerate: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MfaBackupCodesResponse {
    pub backup_codes: Vec<MfaBackupCode>,
}

impl Endpoint for GetMfaBackupCodes {
    type Response = MfaBackupCodesResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/mfa/backup-codes".to_owned())
            .build()
    }
}
