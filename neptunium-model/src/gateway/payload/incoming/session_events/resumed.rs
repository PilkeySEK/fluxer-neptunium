use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resumed {
    pub country_code: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    /// Possible extra data due to this data being undocumented.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
