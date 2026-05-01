use bon::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

/// Creates a new custom theme with CSS styling that can be shared with other users.
#[derive(Clone, Debug, Builder, Serialize)]
pub struct CreateTheme {
    /// CSS text.
    #[builder(into)]
    pub css: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateThemeResponse {
    /// The ID of the created theme.
    pub id: String,
}

impl Endpoint for CreateTheme {
    type Response = CreateThemeResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/themes".to_owned())
            .build()
    }
}
