use neptunium_model::{
    id::{Id, marker::UserMarker},
    user::auth::handoff::HandoffCode,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HandoffComplete {
    pub code: HandoffCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    pub user_id: Id<UserMarker>,
}

impl Endpoint for HandoffComplete {
    type Response = ();
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/auth/handoff/complete".to_owned())
            .build()
    }
}
