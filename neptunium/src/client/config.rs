use neptunium_http::DEFAULT_API_BASE_URL;

pub struct ClientConfig {
    pub(crate) api_base_url: Option<String>,
}

impl ClientConfig {
    #[must_use]
    pub fn api_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = Some(url.into());
        self
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_base_url: Some(DEFAULT_API_BASE_URL.to_owned()),
        }
    }
}
