use std::sync::Arc;

use neptunium_http::client::HttpClient;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) http_client: Arc<HttpClient>,
}

impl Context {
    #[must_use]
    pub fn get_http_client(&self) -> &Arc<HttpClient> {
        &self.http_client
    }
}
