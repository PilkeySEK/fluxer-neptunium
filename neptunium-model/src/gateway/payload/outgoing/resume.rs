use serde::Serialize;
use zeroize::Zeroizing;

#[derive(Serialize, Clone, Debug)]
pub struct Resume {
    /// Same as the token used in the `Identify` payload.
    pub token: Zeroizing<String>,
    pub session_id: Zeroizing<String>,
    /// The last sequence number. The gateway will replay all events after this number.
    pub seq: u64,
}
