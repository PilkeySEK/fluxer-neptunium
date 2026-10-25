use bon::Builder;
use neptunium_model::{
    id::{Id, marker::UserMarker},
    user::relationship::Relationship,
};
use reqwest::Method;
use serde_json::json;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct UpdateRelationshipNickname {
    pub user_id: Id<UserMarker>,
    #[builder(into)]
    pub nickname: Option<String>,
}

impl Endpoint for UpdateRelationshipNickname {
    type Response = Relationship;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PATCH)
            .body(
                json!({
                    "nickname": self.nickname,
                })
                .to_string(),
            )
            .path(format!("/users/@me/relationships/{}", self.user_id))
            .build()
    }
}
