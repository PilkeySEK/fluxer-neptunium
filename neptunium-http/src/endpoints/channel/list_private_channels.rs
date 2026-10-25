use neptunium_model::channel::Channel;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

/// List DM channels.
#[derive(Copy, Clone, Debug)]
pub struct ListPrivateChannels;

impl Endpoint for ListPrivateChannels {
    type Response = Vec<Channel>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/channels".to_owned())
            .build()
    }
}
