use neptunium_model::{
    id::{Id, marker::UserMarker},
    user::{PartialUser, auth::handoff::HandoffCode},
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HandoffStatus {
    pub code: HandoffCode,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum HandoffStatusResponse {
    Completed {
        token: String,
        user_id: Id<UserMarker>,
        user: PartialUser,
    },
    Pending,
    Expired,
}

impl Endpoint for HandoffStatus {
    type Response = HandoffStatusResponse;
    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/auth/handoff/{}/status", self.code))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use neptunium_model::{id::Id, user::flags::PublicUserFlags};

    use super::*;

    #[test]
    fn handoff_status_deserialization() {
        {
            let json = r#"{"status":"pending"}"#;
            let deserialized: HandoffStatusResponse = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, HandoffStatusResponse::Pending);
        }
        {
            let json = r#"{"status":"completed","token":"123","user_id":"123","user":{"discriminator":"0000","flags":0,"id":"123","username":"user"}}"#;
            let deserialized: HandoffStatusResponse = serde_json::from_str(json).unwrap();
            assert_eq!(
                deserialized,
                HandoffStatusResponse::Completed {
                    token: "123".to_owned(),
                    user_id: Id::new(123),
                    user: PartialUser {
                        avatar: None,
                        avatar_color: None,
                        bot: false,
                        discriminator: "0000".to_owned(),
                        flags: PublicUserFlags::empty(),
                        global_name: None,
                        id: Id::new(123),
                        system: false,
                        username: "user".to_owned(),
                        mention_flags: None,
                    }
                }
            );
        }
    }
}
