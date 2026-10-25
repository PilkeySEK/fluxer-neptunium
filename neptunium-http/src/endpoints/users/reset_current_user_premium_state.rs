use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ResetCurrentUserPremiumState;

impl Endpoint for ResetCurrentUserPremiumState {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path("/users/@me/premium/reset".to_owned())
            .build()
    }
}
