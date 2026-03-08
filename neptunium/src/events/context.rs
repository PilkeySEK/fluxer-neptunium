use std::sync::Arc;

use neptunium_http::client::HttpClient;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) api_client: Arc<HttpClient>,
}
