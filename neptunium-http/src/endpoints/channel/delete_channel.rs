use bon::Builder;
use neptunium_model::{
    id::{Id, marker::ChannelMarker},
    user::auth::SudoVerification,
};
use reqwest::{Method, header::HeaderMap};

use crate::{endpoints::Endpoint, request::Request};

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub struct DeleteOrLeaveChannelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_messages: Option<bool>,
}

/// ## Limitations
/// - Deleting a guild channel requires `VIEW_CHANNEL` and `MANAGE_CHANNELS`.
/// - Closing a direct message or leaving a group direct message requires current recipient access.
/// - The personal notes channel cannot be deleted.
/// - Setting delete_messages also requires sudo mode for a user session,
/// satisfied by a valid sudo proof or by the verification fields in the body.
/// A bot token always satisfies it.
#[derive(Builder, Copy, Clone, Debug, PartialEq, Eq)]
pub struct DeleteOrLeaveChannel {
    pub channel_id: Id<ChannelMarker>,
    pub params: DeleteOrLeaveChannelParams,
    pub body: Option<SudoVerification>,
    pub sudo_mode_jwt: Option<String>,
}

impl Endpoint for DeleteOrLeaveChannel {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        let params = serde_qs::to_string(&self.params).unwrap();

        Request::builder()
            .method(Method::DELETE)
            .maybe_body(self.body.map(|body| serde_json::to_string(&body).unwrap()))
            .maybe_headers(self.sudo_mode_jwt.map(|value| {
                let mut headers = HeaderMap::new();
                headers.push("X-Fluxer-Sudo-Mode-JWT".to_owned(), value);
                headers
            }))
            .path(format!("/channels/{}?{}", self.channel_id, params))
            .build()
    }
}
