use neptunium_model::user::data_harvest::DataHarvestResponse;
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Copy, Clone, Debug)]
pub struct GetLatestDataHarvest;

impl Endpoint for GetLatestDataHarvest {
    type Response = Option<DataHarvestResponse>;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::GET)
            .path("/users/@me/harvest/latest".to_owned())
            .build()
    }
}
