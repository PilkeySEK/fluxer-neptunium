use bon::Builder;
use neptunium_model::time::timestamp::{Timestamp, representations::Iso8601};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct GetDataHarvestDownloadUrl {
    #[builder(into)]
    pub harvest_id: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GetDataHarvestDownloadUrlResponse {
    pub download_url: String,
    pub expires_at: Timestamp<Iso8601>,
}

impl Endpoint for GetDataHarvestDownloadUrl {
    type Response = GetDataHarvestDownloadUrlResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/users/@me/harvest/{}/download", self.harvest_id))
            .build()
    }
}
