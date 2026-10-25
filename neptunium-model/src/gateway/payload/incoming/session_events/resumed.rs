use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resumed {
    /// Gateway-side timing breakdown, present only for a staff account.
    #[cfg(feature = "staff_api")]
    _timings_gw: Option<serde_json::Value>,
    /*
    pub country_code: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    /// Possible extra data due to this data being undocumented.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
    */
}
