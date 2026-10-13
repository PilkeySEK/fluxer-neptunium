use bon::Builder;
use serde::Serialize;
use zeroize::Zeroizing;

#[derive(Serialize, Builder, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebauthnAuthenticatorAssertionResponse {
    #[serde(rename = "clientDataJSON")]
    #[builder(into)]
    pub client_data_json: Zeroizing<String>,
    #[builder(into)]
    pub authenticator_data: Zeroizing<String>,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_handle: Option<String>,
}

#[derive(Serialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WebauthnAuthenticatorAttachment {
    CrossPlatform,
    Platform,
}

#[derive(Serialize, Builder, Clone, Debug)]
pub struct WebauthnCredentialsPropertiesOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rk: Option<bool>,
}

#[derive(Serialize, Builder, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebauthnAuthenticationExtensionsClientOutputs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_props: Option<WebauthnCredentialsPropertiesOutput>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmac_create_secret: Option<bool>,
}

#[derive(Serialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WebauthnPublicKeyCredentialType {
    PublicKey,
}

// TODO: check this schema
#[derive(Serialize, Builder, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebAuthnAssertion {
    #[builder(into)]
    pub id: String,
    #[builder(into)]
    pub raw_id: String,
    pub response: WebauthnAuthenticatorAssertionResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_attachment: Option<WebauthnAuthenticatorAttachment>,
    pub client_extension_results: WebauthnAuthenticationExtensionsClientOutputs,
    #[serde(rename = "type")]
    pub r#type: WebauthnPublicKeyCredentialType,
}
