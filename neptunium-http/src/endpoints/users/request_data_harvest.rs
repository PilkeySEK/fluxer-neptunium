use neptunium_model::{
    user::data_harvest::DataHarvestStatus,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct RequestDataHarvest;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct RequestDataHarvestResponse {
    pub harvest_id: String,
    pub status: DataHarvestStatus,
    // ISO8601
    pub created_at: jiff::Timestamp,
}

impl Endpoint for RequestDataHarvest {
    type Response = RequestDataHarvestResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .path("/users/@me/harvest".to_owned())
            .build()
    }
}
