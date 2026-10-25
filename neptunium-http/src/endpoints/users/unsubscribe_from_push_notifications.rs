use bon::Builder;
use reqwest::Method;

use crate::{common_types::SuccessResponse, endpoints::Endpoint, request::Request};

#[derive(Builder, Clone, Debug)]
pub struct UnsubscribeFromPushNotifications {
    #[builder(into)]
    pub subscription_id: String,
}

impl Endpoint for UnsubscribeFromPushNotifications {
    type Response = SuccessResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::DELETE)
            .path(format!(
                "/users/@me/push/subscriptions/{}",
                self.subscription_id
            ))
            .build()
    }
}
