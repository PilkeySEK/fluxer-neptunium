use bon::Builder;
use neptunium_model::id::{Id, marker::UserMarker};
use reqwest::Method;
use serde::Serialize;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct SetUserNote {
    pub user_id: Id<UserMarker>,
    /// Set to `None` to clear the note.
    #[builder(into)]
    pub note: Option<String>,
}

impl Endpoint for SetUserNote {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        #[derive(Serialize)]
        struct SetUserNoteBody {
            // This can be either null or undefined, both work. I choose to set it to null.
            note: Option<String>,
        }

        let body = SetUserNoteBody { note: self.note };

        Request::builder()
            .method(Method::PUT)
            .body(serde_json::to_string(&body).unwrap())
            .path(format!("/users/@me/notes/{}", self.user_id))
            .build()
    }
}
