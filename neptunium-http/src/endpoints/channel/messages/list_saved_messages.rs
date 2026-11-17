use bon::Builder;
use neptunium_model::user::saved_messages::SavedMessage;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};



#[derive(Builder, serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub struct ListSavedMessagesParams {
    /// Maximum number of saved messages to return. 1-100, default 25.
    pub limit: Option<u8>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListSavedMessages {
    pub params: ListSavedMessagesParams,
}

impl Endpoint for ListSavedMessages {
    type Response = Vec<SavedMessage>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .params(serde_qs::to_string(&self.params).unwrap())
            .path("/users/@me/saved-messages".to_owned())
            .build()
    }
}
