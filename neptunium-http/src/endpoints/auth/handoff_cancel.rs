use neptunium_model::user::auth::handoff::HandoffCode;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandoffCancel {
    pub code: HandoffCode,
}

impl Endpoint for HandoffCancel {
    type Response = ();
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!("/auth/handoff/{}", self.code))
            .build()
    }
}
