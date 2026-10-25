use neptunium_model::user::gifts::GiftPrivateResponse;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct ListUserGifts;

impl Endpoint for ListUserGifts {
    type Response = Vec<GiftPrivateResponse>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/gifts".to_owned())
            .build()
    }
}
