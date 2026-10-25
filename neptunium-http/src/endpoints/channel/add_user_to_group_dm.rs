use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, UserMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct AddUserToGroupDm {
    pub channel_id: Id<ChannelMarker>,
    pub user_id: Id<UserMarker>,
}

impl Endpoint for AddUserToGroupDm {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PUT)
            .path(format!(
                "/channels/{}/recipients/{}",
                self.channel_id, self.user_id
            ))
            .build()
    }
}
