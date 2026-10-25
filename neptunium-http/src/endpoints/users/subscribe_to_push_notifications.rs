use bon::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Serialize, Clone, Debug)]
pub struct SubscribeToPushNotifications {
    #[builder(into)]
    pub endpoint: String,
    pub keys: SubscribeToPushNotificationsKeys,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub user_agent: Option<String>,
}

#[derive(Builder, Serialize, Clone, Debug)]
pub struct SubscribeToPushNotificationsKeys {
    /// The P-256 ECDH public key.
    #[builder(into)]
    pub p256dh: Zeroizing<String>,
    /// The authentication secret.
    #[builder(into)]
    pub auth: Zeroizing<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct SubscribeToPushNotificationsResponse {
    pub subscription_id: String,
}

impl Endpoint for SubscribeToPushNotifications {
    type Response = SubscribeToPushNotificationsResponse;

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::POST)
            .body(serde_json::to_string(&self).unwrap())
            .path("/users/@me/push/subscribe".to_owned())
            .build()
    }
}
