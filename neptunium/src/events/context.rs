use std::sync::Arc;

use crate::client::api_info::ApiInfo;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) api_info: Arc<ApiInfo>,
}
