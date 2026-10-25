use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{ChannelMarker, GenericMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct DeletePermissionOverwrite {
    pub channel_id: Id<ChannelMarker>,
    pub overwrite_id: Id<GenericMarker>,
}

impl Endpoint for DeletePermissionOverwrite {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/channels/{}/permissions/{}",
                self.channel_id, self.overwrite_id
            ))
            .build()
    }
}
