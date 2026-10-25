use neptunium_model::user::settings::UserSettings;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct GetUserSettings;

impl Endpoint for GetUserSettings {
    type Response = UserSettings;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/settings".to_owned())
            .build()
    }
}
