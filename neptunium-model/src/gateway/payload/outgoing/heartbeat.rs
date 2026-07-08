use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(transparent)]
pub struct Heartbeat {
    pub last_sequence_number: Option<u64>,
}
