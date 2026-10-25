use bon::Builder;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::user::auth::webauthn::WebAuthnAssertion;

pub mod handoff;
pub mod webauthn;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MfaMethod {
    /// Time-based one-time password authentication via authenticator app.
    Totp,
    /// Security key or biometric authentication.
    Webauthn,
}

#[derive(Builder, Serialize, Clone, Default, PartialEq, Eq)]
pub struct SudoVerification {
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Zeroizing<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<MfaMethod>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<Zeroizing<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<WebAuthnAssertion>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<Zeroizing<String>>,
}

impl std::fmt::Debug for SudoVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SudoVerification { REDACTED }")
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MfaBackupCode {
    pub code: String,
    /// Whether the code has been used.
    pub consumed: bool,
}
