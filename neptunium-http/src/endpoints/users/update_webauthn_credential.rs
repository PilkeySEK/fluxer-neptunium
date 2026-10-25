use bon::Builder;
use neptunium_model::user::auth::SudoVerification;
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct UpdateWebauthnCredential {
    #[builder(into)]
    pub credential_id: String,
    /// New name of the credential.
    #[builder(into)]
    pub name: String,
    pub auth: SudoVerification,
}

impl Endpoint for UpdateWebauthnCredential {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct UpdateWebauthnCredentialBody {
            name: String,
            #[serde(flatten)]
            auth: SudoVerification,
        }

        let body = UpdateWebauthnCredentialBody {
            name: self.name,
            auth: self.auth,
        };

        Request::builder()
            .method(Method::PATCH)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!(
                "/users/@me/mfa/webauthn/credentials/{}",
                self.credential_id
            ))
            .build()
    }
}
