use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct SendSudoSmsCode;

impl Endpoint for SendSudoSmsCode {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path("/users/@me/sudo/mfa/sms/send".to_owned())
            .build()
    }
}
