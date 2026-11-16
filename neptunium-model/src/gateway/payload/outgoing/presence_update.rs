use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::gateway::presence::CustomStatus;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PresenceStatus {
    Online,
    #[serde(rename = "dnd")]
    DoNotDisturb,
    Idle,
    Invisible,
    Offline,
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug)]
pub struct PresenceUpdateOutgoing {
    #[builder(default = PresenceStatus::Online)]
    pub status: PresenceStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_status: Option<CustomStatus>,
    #[builder(default = false)]
    pub afk: bool,
    #[builder(default = false)]
    pub mobile: bool,
}
