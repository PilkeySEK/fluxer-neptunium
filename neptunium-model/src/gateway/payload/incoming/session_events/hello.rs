use crate::time::duration::{Duration, Millis};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Hello {
    pub heartbeat_interval: Duration<Millis>,
}
