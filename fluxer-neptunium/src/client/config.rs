use bon::Builder;
use neptunium_http::DEFAULT_API_BASE_URL;

#[derive(Builder, Debug)]
pub struct ClientConfig {
    #[builder(into)]
    pub api_base_url: Option<String>,
    pub always_propagate_event_errors: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_base_url: Some(DEFAULT_API_BASE_URL.to_owned()),
            always_propagate_event_errors: false,
        }
    }
}
