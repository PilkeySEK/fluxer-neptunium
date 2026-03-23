use bon::Builder;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::user::auth::webauthn::WebauthnAuthenticationResponse;

pub mod webauthn;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MfaMethod {
    /// Time-based one-time password authentication via authenticator app.
    Totp,
    /// One-time password sent via text message.
    Sms,
    /// Security key or biometric authentication.
    Webauthn,
}

// Source: https://github.com/fluxerapp/fluxer/blob/5da26d4ed5ef9f3fe8bef993c0f10ea4f4ee9c1d/packages/schema/src/domains/auth/AuthSchemas.tsx#L92
#[derive(Builder, Serialize, Clone, Debug)]
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
    pub webauthn_response: Option<WebauthnAuthenticationResponse>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<Zeroizing<String>>,
}
