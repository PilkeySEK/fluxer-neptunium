use bon::Builder;
use neptunium_model::{
    id::{Id, marker::UserMarker},
    user::relationship::{Relationship, RelationshipType},
};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

// Known as "Accept or update friend request" in the API docs.
#[derive(Builder, Clone, Debug)]
pub struct UpdateRelationship {
    pub user_id: Id<UserMarker>,
    /// Type of relationship to create.
    pub relationship_type: Option<RelationshipType>,
}

impl Endpoint for UpdateRelationship {
    type Response = Relationship;

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct UpdateRelationshipBody {
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            r#type: Option<RelationshipType>,
        }

        let body = UpdateRelationshipBody {
            r#type: self.relationship_type,
        };

        Request::builder()
            .method(Method::PUT)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!("/users/@me/relationships/{}", self.user_id))
            .build()
    }
}
