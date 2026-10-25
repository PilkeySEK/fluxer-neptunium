use bon::Builder;
use neptunium_model::user::data_harvest::DataHarvestResponse;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct GetDataHarvestStatus {
    #[builder(into)]
    pub harvest_id: String,
}

impl Endpoint for GetDataHarvestStatus {
    type Response = DataHarvestResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path(format!("/users/@me/harvest/{}", self.harvest_id))
            .build()
    }
}
