use std::collections::HashMap;

use bon::Builder;
use neptunium_model::user::saved_messages::SavedMessage;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct ListSavedMessages {
    /// Maximum number of saved messages to return. 1-100, default 25.
    pub limit: Option<u8>,
}

impl Endpoint for ListSavedMessages {
    type Response = Vec<SavedMessage>;

    fn into_request(self) -> crate::request::Request {
        let mut params = HashMap::new();
        if let Some(limit) = self.limit {
            params.insert("limit".to_owned(), limit.to_string());
        }
        Request::builder()
            .method(Method::GET)
            .params(params)
            .path("/users/@me/saved-messages".to_owned())
            .build()
    }
}
