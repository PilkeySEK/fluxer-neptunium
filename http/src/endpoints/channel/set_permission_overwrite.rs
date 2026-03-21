use bon::Builder;
use neptunium_model::{
    channel::PermissionOverwriteEntity,
    guild::permissions::Permissions,
    id::{
        Id,
        marker::{ChannelMarker, GenericMarker},
    },
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct PermissionOverwriteUpdate {
    pub allow: Option<Permissions>,
    pub deny: Option<Permissions>,
    /// Can be either a role ID or a user ID
    pub id: Id<GenericMarker>,
    /// The type of entity this overwrite applies to.
    pub r#type: PermissionOverwriteEntity,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct SetPermissionOverwrite {
    pub channel_id: Id<ChannelMarker>,
    pub overwrite: PermissionOverwriteUpdate,
}

impl Endpoint for SetPermissionOverwrite {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        let mut body = json!({
            "type": self.overwrite.r#type,
        });

        if let Some(allow) = self.overwrite.allow {
            body["allow"] = serde_json::to_value(allow).unwrap();
        }
        if let Some(deny) = self.overwrite.deny {
            body["deny"] = serde_json::to_value(deny).unwrap();
        }

        Request::builder()
            .method(Method::PUT)
            .body(body.to_string())
            .path(format!(
                "/channels/{}/permissions/{}",
                self.channel_id, self.overwrite.id
            ))
            .build()
    }
}
