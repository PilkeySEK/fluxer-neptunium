use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, UserMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct RemoveUserFromGroupDm {
    pub channel_id: Id<ChannelMarker>,
    pub user_id: Id<UserMarker>,
    #[builder(default = false)]
    pub silent: bool,
}

impl Endpoint for RemoveUserFromGroupDm {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        let path = if self.silent {
            format!(
                "/channels/{}/recipients/{}?silent",
                self.channel_id, self.user_id
            )
        } else {
            format!("/channels/{}/recipients/{}", self.channel_id, self.user_id)
        };

        Request::builder().method(Method::DELETE).path(path).build()
    }
}
